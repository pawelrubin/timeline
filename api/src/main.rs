#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod auth;

use std::env;
mod db;
use crate::db::pool::Db;
use db::migrations;

mod quantize;

mod entities;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use entities::{GeodataEntity, GeodataJson};
use quantize::quantize_geodata;

use sea_orm::{entity::*, query::*, ConnectionTrait, DatabaseBackend, EntityTrait, Set, Statement};
use sea_orm_rocket::{Connection, Database};

use rocket::fairing::{self, AdHoc};
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

#[get("/geodata?<on_day>")]
async fn select_many(
    conn: Connection<'_, Db>,
    user: auth::UserClaims,
    on_day: Option<String>, // date in format YYYY-MM-DD
) -> Result<Json<Vec<GeodataJson>>, rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();
    let db_data: Vec<entities::geodata::Model> = GeodataEntity::find()
        .filter(
            Condition::all()
                .add(entities::geodata::Column::Uid.eq(user.sub))
                .add(
                    entities::geodata::Column::Timestamp.gt(match on_day {
                        Some(on_day) => NaiveDate::parse_from_str(&on_day, "%Y-%m-%d")
                            .unwrap_or((Utc::now().naive_utc() - Duration::days(7)).date())
                            .and_hms(0, 0, 0),
                        None => Utc::now().naive_utc() - Duration::days(7),
                    }),
                ),
        )
        .all(db)
        .await
        .expect("could not find geodata");

    let filtered_db_data = quantize_geodata(db_data);
    Ok(Json(
        filtered_db_data.into_iter().map(|x| x.into()).collect(),
    ))
}

#[post("/geodata", format = "json", data = "<geodata>")]
async fn insert_many(
    conn: Connection<'_, Db>,
    user: auth::UserClaims,
    geodata: Json<Vec<GeodataJson>>,
) -> Result<(), rocket::response::Debug<sea_orm::DbErr>> {
    // TODO: handle empty geodata

    let db = conn.into_inner();

    let parsed_geodata_vec = geodata
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

    GeodataEntity::insert_many(parsed_geodata_vec)
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
