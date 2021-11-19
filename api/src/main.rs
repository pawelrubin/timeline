#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

use std::env;

#[launch]
pub fn rocket() -> _ {
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let figment = rocket::Config::figment().merge(("port", port));
    rocket::custom(figment).mount("/", routes![index])
}
