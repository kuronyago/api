use warp::Filter;

mod api;
mod handlers;
mod routes;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let end = get_hello_macro!().with(warp::log("hello"));
    warp::serve(end).run(([0, 0, 0, 0], 8080)).await;
}
