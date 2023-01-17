use diesel::PgConnection;
use rocket::http::Status;
use crate::db;
use crate::models::account::{Account, NewAccount};
use crate::models::response::{Response, ResponseWithStatus};

pub fn signup(data: NewAccount, connection: &PgConnection) -> ResponseWithStatus {
    if Account::signup(data, connection) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from("Signup success."),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
    else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from("Failed signup."),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}