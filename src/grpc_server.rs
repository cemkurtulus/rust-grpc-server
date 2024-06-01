use self::core::service::healthcheck_service::HealthService;
use crate::resource::healthcheck::healthcheck_controller::healthcheck_grpc::healthcheck_service_server::HealthcheckServiceServer;
use tonic::transport::Server;

pub mod resource;
pub mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080".parse()?;
    let service = HealthService::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(HealthcheckServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
