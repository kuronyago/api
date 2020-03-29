use warp::Filter;

mod api;
mod handlers;
mod models;
mod psql;
mod routes;
mod schema;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let api = {
        let create_transfer = create_transfer!();
        let health_check = health_check!();
        let methods = health_check.or(create_transfer);
        methods.with(warp::log("api_v1"))
    };

    warp::serve(api).run(([0, 0, 0, 0], 8080)).await;
}
