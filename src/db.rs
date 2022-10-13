use std::time::SystemTime;

use crate::models::User;
use deadpool_postgres::Client;
use tokio_pg_mapper::{Error, FromTokioPostgresRow};

pub async fn get_user(client: &Client, username: &str) -> Result<User, Error> {
    let _stmt = include_str!("../sql/get_user.sql");
    let _stmt = _stmt.replace("$username", format!("'{}'", &username).as_str());
    let stmt = client.prepare(&_stmt).await?;

    let queried_data = client
        .query(&stmt, &[])
        .await?
        .pop()
        .ok_or(Error::ColumnNotFound)?;

    User::from_row_ref(&queried_data)
}

pub async fn create_user(
    client: &Client,
    username: &str,
    token: &[u8],
    salt: &str,
) -> Result<User, Error> {
    let _stmt = include_str!("../sql/create_user.sql");
    let stmt = client.prepare(_stmt).await?;

    let queried_data = client
        .query(
            &stmt,
            &[&username, &token, &salt, &SystemTime::now()],
        )
        .await?
        .pop()
        .ok_or(Error::ColumnNotFound)?;

    User::from_row_ref(&queried_data)
}
