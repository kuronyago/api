use warp;

pub async fn get_hello(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("Hello, {}", name);

    Ok(warp::reply::html(reply))
}
