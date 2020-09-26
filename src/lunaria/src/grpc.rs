use bevy::prelude::*;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use lunaria_api::counter::count_service_server::CountServiceServer;
use lunaria_api::counter::{GetCountRequest, GetCountResponse, Meta};

use super::Counter;
use chrono::Utc;

pub struct GrpcPlugin;

impl Plugin for GrpcPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(initialize_grpc_server())
            .add_system(process_requests_system.system());
    }
}

fn initialize_grpc_server() -> GrpcQueue {
    let (cmd_tx, cmd_rx) = mpsc::channel::<(GrpcEvent, oneshot::Sender<usize>)>(100);

    let addr = "0.0.0.0:10000".parse().unwrap();

    let server = CountServiceServer::new(CounterService {
        request_queue: Arc::new(Mutex::new(cmd_tx)),
    });

    tokio::spawn(Server::builder().add_service(server).serve(addr));

    GrpcQueue {
        request_queue: cmd_rx,
    }
}

fn process_requests_system(mut counter: ResMut<Counter>, mut queue: ResMut<GrpcQueue>) {
    while let Ok((request, response)) = queue.request_queue.try_recv() {
        match request {
            GrpcEvent::GetCountRequest => {
                counter.requests += 1;
                response.send(counter.count).unwrap();
            }
        }
    }
}

enum GrpcEvent {
    GetCountRequest,
}

struct GrpcQueue {
    request_queue: mpsc::Receiver<(GrpcEvent, oneshot::Sender<usize>)>,
}

type GrpcQueueSender = Arc<Mutex<mpsc::Sender<(GrpcEvent, oneshot::Sender<usize>)>>>;

struct CounterService {
    request_queue: GrpcQueueSender,
}

#[tonic::async_trait]
impl lunaria_api::counter::count_service_server::CountService for CounterService {
    async fn get_count(
        &self,
        _request: Request<GetCountRequest>,
    ) -> Result<Response<GetCountResponse>, Status> {
        let (resp_tx, resp_rx) = oneshot::channel();

        self.request_queue
            .lock()
            .await
            .send((GrpcEvent::GetCountRequest, resp_tx))
            .await
            .ok()
            .unwrap();
        let res = resp_rx.await.unwrap();

        Ok(Response::new(GetCountResponse {
            meta: Some(Meta {
                timestamp: Utc::now().timestamp(),
            }),
            count: res as u64,
        }))
    }
}
