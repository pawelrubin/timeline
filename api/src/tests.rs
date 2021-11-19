use super::*;

#[async_test]
async fn test_basic() {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(rocket()).await.unwrap();
    let response = client.get("/").dispatch().await;

    assert_eq!(response.status(), Status::Ok);

    let s = response.into_string().await;
    assert_eq!(s.unwrap(), "Hello, world!");
}
