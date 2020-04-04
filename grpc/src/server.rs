use rpc::wallet::wallet_server::WalletServer;
use tonic::transport::Server;

mod rpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let wallet_service = rpc::Service::default();

    Server::builder()
        .add_service(WalletServer::new(wallet_service))
        .serve(addr)
        .await?;

    Ok(())
}
