use bevy::app::{AppBuilder, Plugin};
use lunaria_api::lunaria::v1::lunaria_server::LunariaServer;
use tokio_compat_02::FutureExt;
use tonic::transport::Server;

use self::lunaria::Lunaria;

mod lunaria;

pub struct Api;

impl Plugin for Api {
    fn build(&self, _app: &mut AppBuilder) {
        let addr = self.address_or_default().parse().unwrap();

        tokio::spawn(
            Server::builder()
                .add_service(LunariaServer::new(Lunaria::default()))
                .serve(addr)
                .compat(),
        );
    }
}

impl Api {
    fn address_or_default(&self) -> String {
        match std::env::var("LUNARIA_ADDRESS") {
            Ok(address) => address,
            Err(_) => String::from("0.0.0.0:1904"),
        }
    }
}
