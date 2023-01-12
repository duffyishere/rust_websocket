use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Account {
    pub uuid: usize,
    pub email: String,
    pub password: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Insertable)]
pub struct SignupDTO {
    pub uuid: usize,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

impl Account {
    pub fn signup(data: SignupDTO, conn: &PgConnection) -> bool {
        let hashed_pwd = hash(&data.password, DEFAULT_COST).unwrap();
        let account = SignupDTO {
            password: hashed_pwd,
            ..data
        };

        diesel::insert_into(account)
            .values(&account)
            .execute(conn)
            .is_ok()
    }
}