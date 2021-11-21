#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

#[cfg(test)]
mod tests;

mod auth;
use std::env;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement, Set};
use sea_orm::{entity::*};
use sea_orm_rocket::{Connection, Database};
use rocket::serde::{json::Json, uuid::Uuid};

mod pool;
use pool::Db;

mod geodata;
pub use geodata::Entity as Geodata;

#[get("/geodata/<id>")]
async fn get(
    conn: Connection<'_, Db>,
    id: Uuid,
) -> Result<rocket::serde::json::Json<geodata::Model>, rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();
    let geodata: geodata::Model = Geodata::find_by_id(id)
        .one(db)
        .await
        .expect("could not find geodata")
        .unwrap();
    Ok(Json(geodata))
}

#[post("/geodata", format = "json", data = "<json_geodata>")]
async fn post(
    conn: Connection<'_, Db>,
    json_geodata: Json<geodata::Model>,
) -> Result<(), rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();
    let geodata_obj = json_geodata.into_inner();
    geodata::ActiveModel {
        uid: Set(geodata_obj.uid.to_owned()),
        timestamp: Set(geodata_obj.timestamp.to_owned()),
        created_at: Set(geodata_obj.created_at.to_owned()),
        lat: Set(geodata_obj.lat.to_owned()),
        lng: Set(geodata_obj.lng.to_owned()),
        activity: Set(geodata_obj.activity.to_owned()),
        accuracy: Set(geodata_obj.accuracy.to_owned()),
        ..Default::default()
        
    }
    .save(db)
    .await
    .expect("Unable to insert geodata");
    Ok(())
}

#[get("/")]
async fn index(
    conn: Connection<'_, Db>,
) -> Result<String, rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();
    let query_res = db
        .query_one(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT 'Hello, world!' as greeting;".to_owned(),
        ))
        .await?
        .unwrap();
    Ok(query_res.try_get::<String>("", "greeting")?)
}

#[get("/hello")]
async fn hello(user: auth::UserClaims) -> String {
    format!("Hello, {}!", user.email)
}

#[launch]
pub fn rocket() -> _ {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();
    let figment = rocket::Config::figment().merge(("port", port));
    rocket::custom(figment)
        .attach(Db::init())
        .mount("/", routes![index, hello, get, post])
}
