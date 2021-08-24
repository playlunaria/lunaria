use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use bevy::prelude::*;
use lunaria_api::lunaria::v1::game_service_server::GameServiceServer;
use lunaria_api::lunaria::v1::lunaria_service_server::LunariaServiceServer;
use tokio::sync::broadcast::Sender;
use tonic::transport::Server as GrpcServer;

use crate::command::Command;
use crate::plugin::api::game::GameService;
use crate::plugin::api::lunaria::LunariaService;

const ENV_VAR_ADDRESS: &str = "LUNARIA_ADDRESS";

pub mod game;
pub mod lunaria;

pub struct Api {
    command_queue: Sender<Command>,
}

impl Api {
    pub fn new(command_queue: Sender<Command>) -> Self {
        Self { command_queue }
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

impl Plugin for Api {
    fn build(&self, _app: &mut AppBuilder) {
        let address = self.address_or_default();

        tokio::spawn(
            GrpcServer::builder()
                .add_service(GameServiceServer::new(GameService::new(
                    self.command_queue.clone(),
                )))
                .add_service(LunariaServiceServer::new(LunariaService::default()))
                .serve(address),
        );
    }

    fn name(&self) -> &str {
        "plugin/api"
    }
}
