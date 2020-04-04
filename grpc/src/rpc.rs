use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod wallet {
    tonic::include_proto!("wallet");
}

use wallet::{
    wallet_server::Wallet, FindTransferRequest, FindTransferResponse, NewTransferRequest,
    NewTransferResponse,
};

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl Wallet for Service {
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

        let reply = FindTransferResponse {
            code: 0 as i32,
            transfer: None,
        };

        Ok(Response::new(reply))
    }

    async fn new_transfer(
        &self,
        request: Request<NewTransferRequest>,
    ) -> Result<Response<NewTransferResponse>, Status> {
        let req: &NewTransferRequest = request.get_ref();

        let _id: Uuid = match Uuid::from_slice(&req.id) {
            Ok(id) => id,
            Err(err) => {
                println!("parse uuid error: {:?}", err);

                return Ok(Response::new(NewTransferResponse {
                    code: 1 as i32,
                    transfer: None,
                }));
            }
        };

        let reply = NewTransferResponse {
            code: 0,
            transfer: None,
        };

        Ok(Response::new(reply))
    }
}
