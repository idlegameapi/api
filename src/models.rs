use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
    pub username: String,
    pub hashed_password: String,
    pub salt: String,
    pub balance: f64,
    pub collected_timestamp: SystemTime,
}

impl From<User> for UserWithoutSecrets {
    fn from(user: User) -> Self {
        UserWithoutSecrets {
            username: user.username,
            balance: user.balance,
            collected_timestamp: user.collected_timestamp,
        }
    }
}

#[derive(Serialize)]
pub struct UserWithoutSecrets {
    pub username: String,
    pub balance: f64,
    pub collected_timestamp: SystemTime,
}
