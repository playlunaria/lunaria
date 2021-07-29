use std::pin::Pin;

use lunaria_api::lunaria::v1::start_game_response::GameStatus as ApiGameStatus;
use lunaria_api::lunaria::v1::{StartGameRequest, StartGameResponse};
use tokio::sync::watch::Receiver;
use tokio_stream::wrappers::WatchStream;
use tokio_stream::StreamExt;
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status};

use crate::game::GameStatus;

pub struct GameService {
    receiver: Receiver<GameStatus>,
}

impl GameService {
    pub fn new(receiver: Receiver<GameStatus>) -> Self {
        Self { receiver }
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
        let stream = WatchStream::new(self.receiver.clone()).map(|status| match status {
            GameStatus::Loading => Ok(StartGameResponse {
                status: ApiGameStatus::Loading.into(),
            }),
            GameStatus::Running => Ok(StartGameResponse {
                status: ApiGameStatus::Running.into(),
            }),
            GameStatus::Stopped => Ok(StartGameResponse {
                status: ApiGameStatus::Stopped.into(),
            }),
        });

        Ok(Response::new(Box::pin(stream) as Self::StartGameStream))
    }
}
