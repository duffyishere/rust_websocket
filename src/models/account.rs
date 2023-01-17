use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl};
use pwhash::bcrypt;
use rocket::serde::{Deserialize, Serialize};

use crate::models::account;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Account {
    pub uuid: usize,
    pub email: String,
    pub password: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct NewAccount {
    pub uuid: usize,
    pub email: String,
    pub name: String,
    pub password: String,
}

pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

impl Account {
    pub fn signup(data: NewAccount, connection: &PgConnection) -> bool {
        let hashed_password = bcrypt::hash(data.password).unwrap();
        let new_account = NewAccount {
            password: hashed_password,
            ..data
        };

        true
    }
}