#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use serde::{Deserialize, Serialize};

#[get("/")]
fn index() -> String {
    String::from("Hello world")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}