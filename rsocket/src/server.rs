use rsocket_rust::prelude::{EchoRSocket, RSocketFactory};
use rsocket_rust_transport_websocket::WebsocketServerTransport;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let ws_transport: WebsocketServerTransport = WebsocketServerTransport::from("127.0.0.1:8080");

    RSocketFactory::receive()
        .transport(ws_transport)
        .acceptor(|setup, _socket| {
            println!("accept setup: {:?}", setup);
            Ok(Box::new(EchoRSocket))
        })
        .on_start(|| println!("start"))
        .serve()
        .await?;

    Ok(())
}
