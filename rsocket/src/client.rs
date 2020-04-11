use futures::{stream, StreamExt};
use log::{error, info};
use rsocket_rust::prelude::{ClientResponder, EchoRSocket, Payload, RSocket, RSocketFactory};
use rsocket_rust_transport_websocket::WebsocketClientTransport;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    let transport: WebsocketClientTransport = WebsocketClientTransport::from("127.0.0.1:8080");
    let acceptor: ClientResponder = Box::new(|| Box::new(EchoRSocket));
    let setup_payload: Payload = Payload::from("READY!");

    let client = RSocketFactory::connect()
        .acceptor(acceptor)
        .transport(transport)
        .setup(setup_payload)
        .mime_type("text/plain", "text/plain")
        .start()
        .await
        .unwrap();

    let sends: Vec<_> = (0..10)
        .map(|n| {
            let p = Payload::builder()
                .set_data_utf8(&format!("Hello#{}", n))
                .set_metadata_utf8("RUST")
                .build();
            Ok(p)
        })
        .collect();

    let reqs = Box::pin(stream::iter(sends));

    let mut flux = client.request_channel(reqs);

    loop {
        match flux.next().await {
            Some(Ok(v)) => info!("CHANNEL_RESPONSE OK: {:?}", v),
            Some(Err(e)) => error!("CHANNEL_RESPONSE FAILED: {:?}", e),
            None => break,
        }
    }

    client.close();

    Ok(())
}
