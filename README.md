# API examples

# REST JSON API based on [warp] and [diesel]
- echo DATABASE_URL=postgres://username:password@localhost/database > .env
- cargo build --release --bin rest

# GRPC API base on [tonic], [prost] and [hyper]
- cargo build --release --bin client
- cargo build --release --bin server

<!-- links -->
[warp]: https://github.com/seanmonstar/warp
[diesel]: https://github.com/diesel-rs/diesel
[tonic]: https://github.com/hyperium/tonic
[prost]: https://github.com/danburkert/prost
[hyper]: https://github.com/hyperium/hyper