use futures::stream::StreamExt;
use log::info;
use log::warn;
use rsocket_rust::error::RSocketError;
use rsocket_rust::prelude::{
    EchoRSocket, Flux, Mono, Payload, RSocket, RSocketFactory, ServerResponder,
};
use rsocket_rust_transport_websocket::WebsocketServerTransport;
use std::error::Error;
use std::future::Future;
use tokio::io::{self, AsyncWriteExt};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    let transport: WebsocketServerTransport = WebsocketServerTransport::from("127.0.0.1:8080");

    let responder: ServerResponder = Box::new(|setup, _socket| {
        info!("accept setup: {:?}", setup);
        Ok(Box::new(MySocket))
    });

    let on_start: Box<dyn FnMut() + Send + Sync> =
        Box::new(|| info!("+++++++ echo server started! +++++++"));

    RSocketFactory::receive()
        .transport(transport)
        .acceptor(responder)
        .on_start(on_start)
        .serve()
        .await?;

    Ok(())
}

struct MySocket;

impl RSocket for MySocket {
    fn metadata_push(&self, req: Payload) -> Mono<()> {
        info!("{:?}", req);
        Box::pin(async {})
    }

    fn fire_and_forget(&self, req: Payload) -> Mono<()> {
        info!("{:?}", req);
        Box::pin(async {})
    }

    fn request_response(&self, req: Payload) -> Mono<Result<Payload, RSocketError>> {
        info!("{:?}", req);
        Box::pin(async move { Ok(req) })
    }

    fn request_stream(&self, req: Payload) -> Flux<Result<Payload, RSocketError>> {
        info!("{:?}", req);
        Box::pin(futures::stream::iter(vec![Ok(req)]))
    }

    fn request_channel(
        &self,
        mut reqs: Flux<Result<Payload, RSocketError>>,
    ) -> Flux<Result<Payload, RSocketError>> {
        let (sender, receiver) = mpsc::unbounded_channel();

        {
            for i in 0..5 {
                let sender = sender.clone();

                tokio::spawn(async move {
                    let pld = create_payload_from_counter(i);
                    info!("send info back to client: {:?}", pld);

                    sender.send(Ok(pld)).unwrap();
                });
            }
        }

        {
            tokio::spawn(async move {
                while let Some(it) = reqs.next().await {
                    info!("{:?}", it);
                }
            });
        }

        Box::pin(receiver)
    }
}

fn create_payload_from_counter(counter: i32) -> Payload {
    let data_utf8 = format!("counter = {:?}, sender", counter);

    Payload::builder()
        .set_data_utf8(&data_utf8)
        .set_metadata_utf8("hello, metadata")
        .build()
}

fn create_payload() -> Payload {
    let p = Payload::builder()
        .set_data_utf8("hello, sender")
        .set_metadata_utf8("hello, metadata")
        .build();
    p
}
