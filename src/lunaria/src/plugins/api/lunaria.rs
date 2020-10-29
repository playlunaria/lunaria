use tonic::{Request, Response, Status};

use lunaria_api::lunaria::v1::{GetVersionRequest, GetVersionResponse, Version};

#[derive(Clone, Debug, Default)]
pub struct Lunaria {}

#[tonic::async_trait]
impl lunaria_api::lunaria::v1::lunaria_server::Lunaria for Lunaria {
    async fn get_version(
        &self,
        _request: Request<GetVersionRequest>,
    ) -> Result<Response<GetVersionResponse>, Status> {
        let version = Version {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        };

        Ok(Response::new(GetVersionResponse {
            version: Some(version),
        }))
    }
}
