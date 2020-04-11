rsocket\:server:
	cargo build --bin rsocket_server

rsocket\:client:
	cargo build --bin rsocket_client

run\:rsocket\:server:
	RUST_LOG=info target/debug/rsocket_server

run\:rsocket\:client:
	RUST_LOG=info target/debug/rsocket_client