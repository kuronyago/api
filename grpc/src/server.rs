use tonic::{transport::Server, Request, Response, Status};

use uuid::Uuid;
use wallet::wallet_server::{Wallet, WalletServer};
use wallet::{FindTransferRequest, FindTransferResponse, Transfer};

pub mod wallet {
    tonic::include_proto!("wallet");
}

#[derive(Debug, Default)]
pub struct MyWallet {}

#[tonic::async_trait]
impl Wallet for MyWallet {
    async fn find_transfer(
        &self,
        request: Request<FindTransferRequest>,
    ) -> Result<Response<FindTransferResponse>, Status> {
        let req: &FindTransferRequest = request.get_ref();

        let external: Uuid = match Uuid::from_slice(&req.external) {
            Ok(id) => id,
            Err(err) => {
                println!("parse uuid error: {:?}", err);

                return Ok(Response::new(wallet::FindTransferResponse {
                    code: 1 as i32,
                    transfer: None,
                }));
            }
        };

        println!("external: {:?}", external);

        let reply = wallet::FindTransferResponse {
            code: 0 as i32,
            transfer: None,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let wallet_service = MyWallet::default();

    Server::builder()
        .add_service(WalletServer::new(wallet_service))
        .serve(addr)
        .await?;

    Ok(())
}
