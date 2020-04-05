use rsocket_rust::prelude::*;
use rsocket_rust_transport_websocket::WebsocketServerTransport;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    RSocketFactory::receive()
        .transport(WebsocketServerTransport::from("127.0.0.1:8080"))
        .acceptor(|setup, _socket| {
            println!("accept setup: {:?}", setup);
            Ok(Box::new(EchoRSocket))
        })
        .on_start(|| println!("start"))
        .serve()
        .await?;

    Ok(())
}
