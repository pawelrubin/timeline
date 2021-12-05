use super::rocket;

use rocket::http::{Header, Status};
use rocket::local::asynchronous::Client;

static TEST_USER_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjQ3OTg5ZTU4ZWU1ODM4OTgzZDhhNDQwNWRlOTVkYTllZTZmNWVlYjgiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL3NlY3VyZXRva2VuLmdvb2dsZS5jb20vdGltZWxpbmUtMzMxNzE4IiwiYXVkIjoidGltZWxpbmUtMzMxNzE4IiwiYXV0aF90aW1lIjoxNjM4NzIwNDA5LCJ1c2VyX2lkIjoiUXR6Y09rVk9GRVMzN2syVERXNmdic2hLMVJyMSIsInN1YiI6IlF0emNPa1ZPRkVTMzdrMlREVzZnYnNoSzFScjEiLCJpYXQiOjE2Mzg3MjA0MDksImV4cCI6MTYzODcyNDAwOSwiZW1haWwiOiJ0ZXN0LnVzZXJAdGltZWxpbmUuY29tIiwiZW1haWxfdmVyaWZpZWQiOmZhbHNlLCJmaXJlYmFzZSI6eyJpZGVudGl0aWVzIjp7ImVtYWlsIjpbInRlc3QudXNlckB0aW1lbGluZS5jb20iXX0sInNpZ25faW5fcHJvdmlkZXIiOiJwYXNzd29yZCJ9fQ.c8KLOFyBu0eWsYOeuTGNsmWKP6FZtY4iDTmnHKLzHFkRkFeBgiENXpEX-ZRNshRpDUesm5JGaeJ8m23IT-K0RjxaLr37H173aqBC-juXrWjNKBk-fhrhKAG0wQqDYGIDxfTxPzh6fge7dTgem2H_nTQJ7AZ3s_j5Ja5ysw3_BwE9siGOokR4IAdsP6_cIQNy13vFa1OfcTDaBbJGxuX2yFWhEWPC_hG3XXYrHV4fOgnRl5LshTx7NzmfneKg6PpIcoX0OPJyy8v8GtxnZg59n-ZaxgUg5DPPSoyspFk4sxWzWoRj8uoz4wktOi-BwocYlM4bfugH7yDU67LJjc_vOA";
static TEST_USER_EMAIL: &str = "test.user@timeline.com";

#[async_test]
async fn test_basic() {
    let client = Client::tracked(rocket()).await.unwrap();
    let response = client.get("/").dispatch().await;

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().await.unwrap();
    assert_eq!(body, "Hello, world!");
}

#[async_test]
async fn test_auth_required() {
    let client = Client::tracked(rocket()).await.unwrap();
    let response = client.get("/hello").dispatch().await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[async_test]
async fn test_auth_ok() {
    let client = Client::tracked(rocket()).await.unwrap();
    let response = client
        .get("/hello")
        .header(Header::new(
            "Authorization",
            format!("Bearer {}", TEST_USER_TOKEN),
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().await.unwrap();
    assert_eq!(body, format!("Hello, {}!", TEST_USER_EMAIL));
}

#[async_test]
async fn test_geodata_flow_no_auth() {
    let client = Client::tracked(rocket()).await.unwrap();
    let response = client.get("/geodata").dispatch().await;

    assert_eq!(response.status(), Status::Unauthorized);
}
