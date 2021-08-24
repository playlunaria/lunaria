use lunaria_api::lunaria::v1::{StartGameRequest, StartGameResponse};
use tokio::sync::broadcast::Sender;
use tonic::{Request, Response, Status};

use crate::command::Command;

pub struct GameService {
    command_queue: Sender<Command>,
}

impl GameService {
    pub fn new(command_queue: Sender<Command>) -> Self {
        Self { command_queue }
    }
}

#[tonic::async_trait]
impl lunaria_api::lunaria::v1::game_service_server::GameService for GameService {
    async fn start_game(
        &self,
        _request: Request<StartGameRequest>,
    ) -> Result<Response<StartGameResponse>, Status> {
        match self.command_queue.send(Command::StartGame) {
            Ok(_) => Ok(Response::new(StartGameResponse {})),
            Err(error) => Err(Status::internal(error.to_string())),
        }
    }
}
