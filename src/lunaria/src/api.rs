use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use lunaria_api::lunaria::v1::game_service_server::GameServiceServer;
use lunaria_api::lunaria::v1::lunaria_service_server::LunariaServiceServer;
use tonic::transport::Server as GrpcServer;

use crate::api::game::GameService;
use crate::api::lunaria::LunariaService;
use crate::engine::{CommandQueue, EventQueue};

const ENV_VAR_ADDRESS: &str = "LUNARIA_ADDRESS";

pub mod game;
pub mod lunaria;

pub struct Api {
    #[allow(dead_code)] // TODO: Remove when switching to synchronous start command
    command_queue: CommandQueue,
    event_queue: EventQueue,
}

impl Api {
    pub fn new(command_queue: CommandQueue, event_queue: EventQueue) -> Self {
        Self {
            command_queue,
            event_queue,
        }
    }

    pub async fn serve(self) -> Result<(), tonic::transport::Error> {
        let address = self.address_or_default();

        GrpcServer::builder()
            .add_service(GameServiceServer::new(GameService::new(
                self.event_queue.clone(),
            )))
            .add_service(LunariaServiceServer::new(LunariaService::default()))
            .serve(address)
            .await
    }

    fn address_or_default(&self) -> SocketAddr {
        if let Ok(address_string) = std::env::var(ENV_VAR_ADDRESS) {
            if let Ok(address) = SocketAddr::from_str(&address_string) {
                return address;
            }
        }

        SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 1904)
    }
}
