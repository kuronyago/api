use rsocket_rust::prelude::*;
use rsocket_rust_transport_websocket::WebsocketClientTransport;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cli = RSocketFactory::connect()
        .acceptor(|| Box::new(EchoRSocket))
        .transport(WebsocketClientTransport::from("127.0.0.1:8080"))
        .setup(Payload::from("READY!"))
        .mime_type("text/plain", "text/plain")
        .start()
        .await
        .unwrap();
    let req = Payload::builder()
        .set_data_utf8("Hello World!")
        .set_metadata_utf8("Rust")
        .build();
    let res = cli.request_response(req).await.unwrap();
    println!("got: {:?}", res);
    cli.close();

    Ok(())
}
