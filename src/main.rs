#[macro_use]
extern crate rocket;
extern crate core;

use rocket::http::Status;
use rocket::Response;
use rocket::response::status;
use rocket::serde::json::Json;

mod models;

#[get("/")]
fn index() -> String {
    String::from("Hello world")
}

#[post("/signup", data = "<user>")]
pub fn signup(user: Json<SignupDto>, connection: DbConn) {
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, signup])
}