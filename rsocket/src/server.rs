use log::info;
use rsocket_rust::prelude::{EchoRSocket, RSocketFactory, ServerResponder};
use rsocket_rust_transport_websocket::WebsocketServerTransport;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let ws_transport: WebsocketServerTransport = WebsocketServerTransport::from("127.0.0.1:8080");

    let responder: ServerResponder = Box::new(|setup, _socket| {
        println!("accept setup: {:?}", setup);
        Ok(Box::new(EchoRSocket))
    });

    let on_start = Box::new(|| info!("start"));

    RSocketFactory::receive()
        .transport(ws_transport)
        .acceptor(responder)
        .on_start(on_start)
        .serve()
        .await?;

    Ok(())
}
