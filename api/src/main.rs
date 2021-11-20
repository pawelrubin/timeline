#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use std::env;

use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use sea_orm_rocket::{Connection, Database};

mod pool;
use pool::Db;

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

#[launch]
pub fn rocket() -> _ {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();
    let figment = rocket::Config::figment().merge(("port", port));
    rocket::custom(figment)
        .attach(Db::init())
        .mount("/", routes![index])
}
