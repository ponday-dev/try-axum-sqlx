use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub name: String,
}
