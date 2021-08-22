use getset::Getters;
use tokio::runtime::Runtime;
use tokio::sync::broadcast::Sender;

use crate::api::Api;
use crate::command::Command;
use crate::error::{Code, Error, ErrorKind, Result};

#[derive(Getters)]
pub struct Engine {
    #[getset(get = "pub")]
    runtime: Runtime,
}

pub type CommandQueue = Sender<Command>;

impl Engine {
    pub fn new(command_queue: CommandQueue) -> Result<Self> {
        let runtime = initialize_runtime()?;
        let api = Api::new(command_queue);

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
