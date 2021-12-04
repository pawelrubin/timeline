#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod auth;

use std::env;

mod db;
use db::migrations;
use crate::db::pool::Db;

mod entities;
use entities::Geodata;

use sea_orm::{ConnectionTrait, EntityTrait, DatabaseBackend, Statement, Set, ActiveModelTrait};
use sea_orm_rocket::{Connection, Database};

use rocket::{Build, Rocket};
use rocket::serde::{json::Json,};
use rocket::fairing::{self, AdHoc};



#[get("/geodata/<id>")]
async fn get(
    conn: Connection<'_, Db>,
    id: i64,
) -> Result<Json<entities::geodata::Model>, rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();
    let geodata: entities::geodata::Model = Geodata::find_by_id(id)
        .one(db)
        .await
        .expect("could not find geodata")
        .unwrap();
    Ok(Json(geodata))
}

#[post("/geodata", format = "json", data = "<wrapped_geodata>")]
async fn post(
    conn: Connection<'_, Db>,
    wrapped_geodata: Json<entities::geodata::InputData>,
) -> Result<(), rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();
    let geodata = wrapped_geodata.clone().into_inner();
    entities::geodata::ActiveModel {
        uid: Set(geodata.uid.to_owned()),
        timestamp: Set(geodata.timestamp.to_owned()),
        created_at: Set(geodata.created_at.to_owned()),
        lat: Set(geodata.lat.to_owned()),
        lng: Set(geodata.lng.to_owned()),
        accuracy: Set(geodata.accuracy.to_owned()),
        activity: Set(geodata.activity.to_owned()),
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


async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migrations::create_tables(conn).await;
    Ok(rocket)
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
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![index, hello, get, post])
}
