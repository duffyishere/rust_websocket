use rocket::http::Status;
use crate::models::account::{Account, SignupDTO};
use crate::models::response::{Response, ResponseWithStatus};

pub fn signup(data: SignupDTO, connection: DbConn) -> ResponseWithStatus {
    if Account::signup(data, connection) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
    else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}