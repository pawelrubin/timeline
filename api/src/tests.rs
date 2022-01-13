use super::entities::geodata::{Activity, GeodataJson};
use super::rocket;

use chrono::Utc;
use rocket::http::{ContentType, Header, Status};
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

#[async_test]
async fn test_get_geodata() {
    // TODO: Use a test-only database.
    // TODO: Save some data in the database and check that it's returned.
    let client = Client::tracked(rocket()).await.unwrap();
    let response = client
        .get("/geodata")
        .header(Header::new(
            "Authorization",
            format!("Bearer {}", TEST_USER_TOKEN),
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[async_test]
async fn test_post_geodata() {
    // TODO: Use a test-only database.
    // TODO: Check that the data was actually saved.
    let client = Client::tracked(rocket()).await.unwrap();
    let data = GeodataJson {
        lat: 1.0,
        lng: 2.0,
        timestamp: Utc::now().naive_utc(),
        accuracy: 10,
        activity: Activity::OnFoot,
    };
    let response = client
        .post("/geodata")
        .body(format!("[{}]", serde_json::to_string(&data).unwrap()))
        .header(ContentType::JSON)
        .header(Header::new(
            "Authorization",
            format!("Bearer {}", TEST_USER_TOKEN),
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_quantize() {
    use chrono::NaiveDate;

    use crate::entities::{GeodataActivity, GeodataModel};
    use crate::quantize::quantize_geodata;
    let geo_vec = vec![
        GeodataModel {
            id: 0,
            uid: "a".to_string(),
            timestamp: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            lat: 50.0,
            lng: 50.0,
            accuracy: 10,
            activity: GeodataActivity::OnFoot,
        },
        GeodataModel {
            id: 1,
            uid: "a".to_string(),
            timestamp: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 20, 11),
            created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            lat: 50.01,
            lng: 50.01,
            accuracy: 10,
            activity: GeodataActivity::OnFoot,
        },
        GeodataModel {
            id: 2,
            uid: "a".to_string(),

            timestamp: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 30, 11),
            created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            lat: 50.020005,
            lng: 50.0200,
            accuracy: 10,
            activity: GeodataActivity::OnFoot,
        },
        GeodataModel {
            id: 3,
            uid: "a".to_string(),
            timestamp: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 40, 11),
            created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            lat: 50.03,
            lng: 50.03,
            accuracy: 10,
            activity: GeodataActivity::OnFoot,
        },
        GeodataModel {
            id: 4,
            uid: "a".to_string(),
            timestamp: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 50, 11),
            created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            lat: 50.0252,
            lng: 50.025,
            accuracy: 10,
            activity: GeodataActivity::OnFoot,
        },
        GeodataModel {
            id: 5,
            uid: "a".to_string(),
            timestamp: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 50, 11),
            created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            lat: 52.55,
            lng: 55.5,
            accuracy: 10,
            activity: GeodataActivity::OnFoot,
        },
    ];
    let result = quantize_geodata(geo_vec.to_owned());
    assert_eq!(
        result,
        vec![vec![
            geo_vec[0].to_owned(),
            geo_vec[3].to_owned(),
            geo_vec[4].to_owned(),
            geo_vec[5].to_owned()
        ]]
    )
}
#[test]
fn test_distance_from_line() {
    use crate::quantize::calculate_distance_from_line;

    let start = [0., 0.];
    let end = [3., 3.];
    let point = [3., 0.];

    assert_eq!(calculate_distance_from_line(point, start, end), 236145.3806450594)
}
