use warp::{filters::BoxedFilter, path, Filter};

use super::models::NewTransferCommand;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / ..).boxed()
}

pub fn create() -> warp::filters::BoxedFilter<(NewTransferCommand,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post()
        .and(path_prefix())
        .and(warp::path("transfer"))
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn health_check() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("check"))
        .and(warp::path::end())
        .boxed()
}
