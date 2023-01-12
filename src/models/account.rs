#[derive(Serialize, Deserialize)]
pub struct Account {
    pub uuid: usize,
    pub email: String,
    pub password: String,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

impl Account {
    // TODO: DB Connect logic
}