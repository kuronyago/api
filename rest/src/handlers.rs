use serde_derive::Serialize;
use warp;

use super::models::{NewTransfer, NewTransferCommand, Transfer};
use super::psql::POOL;

#[derive(Serialize)]
struct HealthCheck {
    pub message: String,
}

pub async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&HealthCheck {
        message: String::from("ok"),
    }))
}

pub async fn create(cmd: NewTransferCommand) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();

    let resp: Result<Transfer, diesel::result::Error> = NewTransfer::create(&cmd, &conn);

    let reply = match resp {
        Ok(post) => post,
        Err(err) => {
            println!("{:#?}", err);
            return Err(warp::reject::not_found());
        }
    };

    Ok(warp::reply::json(&reply))
}
