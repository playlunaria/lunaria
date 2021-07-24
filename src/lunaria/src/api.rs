use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use lunaria_api::lunaria::v1::lunaria_service_server::LunariaServiceServer;
use tonic::transport::Server as GrpcServer;

use crate::api::lunaria::LunariaService;

const ENV_VAR_ADDRESS: &str = "LUNARIA_ADDRESS";

pub mod lunaria;

pub struct Api;

impl Api {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self) {
        let address = self.address_or_default();

        tokio::spawn(
            GrpcServer::builder()
                .add_service(LunariaServiceServer::new(LunariaService::default()))
                .serve(address),
        );
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

impl Default for Api {
    fn default() -> Self {
        Self {}
    }
}
