use std::time::SystemTime;

use crate::models::User;
use deadpool_postgres::Client;
use tokio_pg_mapper::{Error, FromTokioPostgresRow};

pub async fn get_user_by_username(client: &Client, username: &str) -> Result<User, Error> {
    let _stmt = include_str!("../sql/get_user_by_username.sql");
    let _stmt = _stmt.replace("$username", format!("'{}'", &username).as_str());
    let stmt = client.prepare(&_stmt).await?;

    let queried_data = client
        .query(&stmt, &[])
        .await?
        .pop()
        .ok_or(Error::ColumnNotFound)?;

    User::from_row_ref(&queried_data)
}

pub async fn get_user_by_token(client: &Client, token: &str) -> Result<User, Error> {
    let _stmt = include_str!("../sql/get_user_by_token.sql");
    let _stmt = _stmt.replace("$token", format!("'{}'", &token).as_str());
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
    token: &str,
    salt: &str,
) -> Result<User, Error> {
    let _stmt = include_str!("../sql/create_user.sql");
    let stmt = client.prepare(_stmt).await?;

    let queried_data = client
        .query(
            &stmt,
            &[&username, &token, &salt],
        )
        .await?
        .pop()
        .ok_or(Error::ColumnNotFound)?;

    User::from_row_ref(&queried_data)
}

pub async fn update_collect_user(
    client: &Client,
    username: &str,
    balance: f64,
) -> Result<User, Error> {
    let _stmt = include_str!("../sql/update_collect_user.sql");
    let stmt = client.prepare(_stmt).await?;

    let queried_data = client
        .query(
            &stmt,
            &[&username, &balance, &SystemTime::now()],
        )
        .await?
        .pop()
        .ok_or(Error::ColumnNotFound)?;

    User::from_row_ref(&queried_data)
}

pub async fn update_upgrade_user(
    client: &Client,
    username: &str,
    balance: f64,
    level: i32,
) -> Result<User, Error> {
    let _stmt = include_str!("../sql/update_upgrade_user.sql");
    let stmt = client.prepare(_stmt).await?;

    let queried_data = client
        .query(
            &stmt,
            &[&username, &balance, &level],
        )
        .await?
        .pop()
        .ok_or(Error::ColumnNotFound)?;

    User::from_row_ref(&queried_data)
}
