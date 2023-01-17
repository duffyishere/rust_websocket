#[macro_use]
extern crate rocket;
extern crate core;

use diesel::PgConnection;
use rocket::http::Status;
use rocket::Response;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::models::account::NewAccount;
use crate::models::response::ResponseWithStatus;

mod models;
mod services;
mod db;

#[get("/")]
fn index() -> String {
    String::from("Hello world")
}

#[post("/signup", data = "<user>")]
pub fn signup(user: Json<NewAccount>, connection: &PgConnection) {
    services::account_service::signup(user.0, &connection);
}

fn main() {
    db::rocket().0.launch();
}