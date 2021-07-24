use getset::Getters;
use tokio::runtime::Runtime;

use crate::api::Api;
use crate::error::{Code, Error, ErrorKind, Result};

#[derive(Getters)]
pub struct Engine {
    #[getset(get = "pub")]
    runtime: Runtime,
}

impl Engine {
    pub fn new() -> Result<Self> {
        let runtime = initialize_runtime()?;
        let api = initialize_api();

        runtime.spawn(api.serve());

        Ok(Self { runtime })
    }
}

fn initialize_api() -> Api {
    Api::new()
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
