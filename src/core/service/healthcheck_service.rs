use tonic::{Request, Response, Status};

use crate::resource::healthcheck::healthcheck_controller::healthcheck_grpc::healthcheck_service_server::HealthcheckService;
use crate::resource::healthcheck::healthcheck_controller::healthcheck_grpc::{PingRequest, PongResponse};

#[derive(Debug, Default)]
pub struct HealthService {
}

#[tonic::async_trait]
impl HealthcheckService for HealthService {
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PongResponse>, Status> {
        println!("{}", request.into_inner().request);
        let reply = PongResponse { response : "Pong".parse().unwrap() };
        Ok(Response::new(reply))
    }
}
