use getset::Getters;
use tokio::runtime::Runtime;
use tokio::sync::watch::Receiver;

use crate::api::Api;
use crate::error::{Code, Error, ErrorKind, Result};
use crate::game::GameStatus;

#[derive(Getters)]
pub struct Engine {
    #[getset(get = "pub")]
    runtime: Runtime,
}

impl Engine {
    pub fn new(game_status_receiver: Receiver<GameStatus>) -> Result<Self> {
        let runtime = initialize_runtime()?;
        let api = Api::new(game_status_receiver);

        runtime.spawn(api.serve());

        Ok(Self { runtime })
    }
}

fn initialize_runtime() -> Result<Runtime> {
    match Runtime::new() {
        Ok(runtime) => Ok(runtime),
        Err(error) => Err(Error::new(
            Code::new("LUN0001"),
            ErrorKind::Runtime,
            &format!("Failed to initialize the runtime. {}", error),
        )),
    }
}
