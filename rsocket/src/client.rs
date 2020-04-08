use rsocket_rust::prelude::{ClientResponder, EchoRSocket, Payload, RSocket, RSocketFactory};
use rsocket_rust_transport_websocket::WebsocketClientTransport;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // let echo = Box::new(EchoRSocket);
    let acceptor: ClientResponder = Box::new(|| Box::new(EchoRSocket));

    let client = RSocketFactory::connect()
        .acceptor(acceptor)
        .transport(WebsocketClientTransport::from("127.0.0.1:8080"))
        .setup(Payload::from("READY!"))
        .mime_type("text/plain", "text/plain")
        .start()
        .await
        .unwrap();

    let req: Payload = Payload::builder()
        .set_data_utf8("Hello World!")
        .set_metadata_utf8("Rust")
        .build();

    let res = client.request_response(req).await.unwrap();

    println!("got: {:?}", res);

    client.close();

    Ok(())
}
