use super::*;

#[tokio::test]
async fn health_check() {
    let res = warp::test::request()
        .method("GET")
        .path("/api/v1/check")
        .reply(&health_check!())
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    assert_eq!(res.body(), "{\"message\":\"ok\"}");
}
