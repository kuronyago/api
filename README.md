# API examples

# REST JSON API based on [warp] and [diesel]
- echo DATABASE_URL=postgres://username:password@localhost/database > rest/.env
- cargo build --release --bin rest

# GRPC API base on [tonic], [prost] and [hyper]
- echo DATABASE_URL=postgres://username:password@localhost/database > grpc/.env
- cargo build --release --bin grpc_client
- cargo build --release --bin grpc_server

# RSocket API base on [rsocket_rust]
- cargo build --release --bin rsocket_client
- cargo build --release --bin rsocket_server

<!-- links -->
[warp]: https://github.com/seanmonstar/warp
[diesel]: https://github.com/diesel-rs/diesel
[tonic]: https://github.com/hyperium/tonic
[prost]: https://github.com/danburkert/prost
[hyper]: https://github.com/hyperium/hyper
[rsocket_rust]: https://github.com/rsocket/rsocket-rust