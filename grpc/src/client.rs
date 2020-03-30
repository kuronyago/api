use uuid::Uuid;
use wallet::wallet_client::WalletClient;
use wallet::FindTransferRequest;

pub mod wallet {
    tonic::include_proto!("wallet");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WalletClient::connect("http://[::1]:50051").await?;

    let external = Uuid::parse_str("0000002a-000c-0005-0c03-0938362b0809")?;

    let request = tonic::Request::new(FindTransferRequest {
        external: external.as_bytes().to_vec(),
    });

    let response = client.find_transfer(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
