use log::info;
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

    let request_payload: Payload = Payload::builder()
        .set_data_utf8("this is data")
        .set_metadata_utf8("this is meta-data")
        .build();

    let res = client.request_response(request_payload).await.unwrap();

    info!("got: {:?}", res);

    client.close();

    Ok(())
}
