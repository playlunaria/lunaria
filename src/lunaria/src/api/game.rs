use std::pin::Pin;

use lunaria_api::lunaria::v1::start_game_response::GameStatus as ApiGameStatus;
use lunaria_api::lunaria::v1::{StartGameRequest, StartGameResponse};
use tokio_stream::wrappers::WatchStream;
use tokio_stream::StreamExt;
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status};

use crate::engine::EventQueue;
use crate::event::Event;

pub struct GameService {
    event_queue: EventQueue,
}

impl GameService {
    pub fn new(event_queue: EventQueue) -> Self {
        Self { event_queue }
    }
}

#[tonic::async_trait]
impl lunaria_api::lunaria::v1::game_service_server::GameService for GameService {
    type StartGameStream =
        Pin<Box<dyn Stream<Item = Result<StartGameResponse, Status>> + Send + Sync + 'static>>;

    async fn start_game(
        &self,
        _request: Request<StartGameRequest>,
    ) -> Result<Response<Self::StartGameStream>, Status> {
        let stream = WatchStream::new(self.event_queue.clone()).map(|status| match status {
            Event::None => Ok(StartGameResponse {
                status: ApiGameStatus::Unspecified.into(),
            }),
            Event::GameStarted => Ok(StartGameResponse {
                status: ApiGameStatus::Running.into(),
            }),
            Event::GameFinished => Ok(StartGameResponse {
                status: ApiGameStatus::Stopped.into(),
            }),
        });

        Ok(Response::new(Box::pin(stream) as Self::StartGameStream))
    }
}
