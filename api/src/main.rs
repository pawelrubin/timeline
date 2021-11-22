#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

#[cfg(test)]
mod tests;

mod auth;
use std::env;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
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
    rocket::custom(figment).mount("/", routes![index, hello])
}
