use super::*;

#[tokio::test]
async fn get_hello() {
    let res = warp::test::request()
        .method("GET")
        .path("/hello/42")
        .reply(&get_hello_macro!())
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    assert_eq!(res.body(), "Hello, 42");
}
