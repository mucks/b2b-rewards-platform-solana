use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignUpDto {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}
