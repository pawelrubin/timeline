#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
