#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod auth;

use std::env;

mod db;
use crate::db::pool::Db;
use db::migrations;

mod entities;
use entities::Geodata;

use chrono::{Duration, Utc};

use sea_orm::{entity::*, query::*, ConnectionTrait, DatabaseBackend, EntityTrait, Set, Statement};
use sea_orm_rocket::{Connection, Database};

use rocket::fairing::{self, AdHoc};
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

#[get("/geodata")]
async fn select_many(
    conn: Connection<'_, Db>,
    user: auth::UserClaims,
) -> Result<Json<Vec<entities::geodata::Model>>, rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();
    let geodata_vec: Vec<entities::geodata::Model> = Geodata::find()
        .filter(
            Condition::all()
                .add(entities::geodata::Column::Uid.eq(user.sub))
                .add(
                    entities::geodata::Column::Timestamp
                        .gt(Utc::now().naive_utc() - Duration::days(7)),
                ),
        )
        .all(db)
        .await
        .expect("could not find geodata");
    Ok(Json(geodata_vec))
}

#[post("/geodata", format = "json", data = "<wrapped_geodata_vec>")]
async fn insert_many(
    conn: Connection<'_, Db>,
    user: auth::UserClaims,
    wrapped_geodata_vec: Json<Vec<entities::geodata::InputData>>,
) -> Result<(), rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();
    let geodata_vec = wrapped_geodata_vec.clone().into_inner();
    let parsed_geodata_vec = geodata_vec
        .iter()
        .map(|geodata| entities::geodata::ActiveModel {
            uid: Set(user.sub.to_owned()),
            timestamp: Set(geodata.timestamp.to_owned()),
            created_at: Set(Utc::now().naive_utc()),
            lat: Set(geodata.lat.to_owned()),
            lng: Set(geodata.lng.to_owned()),
            accuracy: Set(geodata.accuracy.to_owned()),
            activity: Set(geodata.activity.to_owned()),
            ..Default::default()
        });
    entities::geodata::Entity::insert_many(parsed_geodata_vec)
        .exec(db)
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
        .mount("/", routes![index, hello, insert_many, select_many])
}
