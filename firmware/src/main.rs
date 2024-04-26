use com::proto::rpc_servo_driver_api_server::RpcServoDriverApiServer;
use pose::ServoDriverApi;
use tonic::transport::Server;

pub(crate) mod pose;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = ServoDriverApi::new().await?;

    let server = RpcServoDriverApiServer::new(api);

    Server::builder()
        .add_service(server)
        .serve("[::1]:50051".parse()?)
        .await?;
    
    Ok(())
}
