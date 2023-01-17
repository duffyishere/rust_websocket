use diesel::PgConnection;
use rocket::fairing::AdHoc;
use rocket::{Phase, Rocket};
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use crate::models::account::NewAccount;
use crate::services;

#[database("postgres_database")]
pub struct DbConn(PgConnection);

#[get("/")]
fn index() -> String {
    String::from("Hello world")
}

#[post("/signup", data = "<user>")]
pub fn signup(user: Json<NewAccount>, connection: &PgConnection) {
    services::account_service::signup(user.0, &connection);
}

pub fn rocket() -> (Rocket<P>, Option<DbConn>) {
    let rocket = rocket.build()
        .attach(DbConn::fairing())
        .mount("/", routes![index, signup]);

    let conn = if cfg!(test) {
        DbConn::get_one(&rocket)
    } else {
        None
    };

    (rocket, conn)
}
