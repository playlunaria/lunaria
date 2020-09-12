use bevy::app::{AppBuilder, Plugin};
use tokio::task::JoinHandle;
use tonic::transport::{Error, Server};
use tonic::{Request, Response, Status};

use lunaria_api::counter::count_service_server::{CountService, CountServiceServer};
use lunaria_api::counter::{GetCountRequest, GetCountResponse};

pub struct GrpcPlugin;

impl Plugin for GrpcPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(initialize_grpc_server());
    }
}

fn initialize_grpc_server() -> JoinHandle<Result<(), Error>> {
    let addr = "0.0.0.0:10000".parse().unwrap();

    let server = CountServiceServer::new(Counter);

    tokio::spawn(Server::builder().add_service(server).serve(addr))
}

struct Counter;

#[tonic::async_trait]
impl CountService for Counter {
    async fn get_count(
        &self,
        _request: Request<GetCountRequest>,
    ) -> Result<Response<GetCountResponse>, Status> {
        Ok(Response::new(GetCountResponse { count: 0 }))
    }
}
