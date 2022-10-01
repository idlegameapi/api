use crate::models::User;
use deadpool_postgres::Client;
use tokio_pg_mapper::{Error, FromTokioPostgresRow};

pub async fn get_userdata(client: &Client, token: &str) -> Result<User, Error> {
    let _stmt = include_str!("../sql/get_user.sql");
    let _stmt = _stmt.replace("$token", format!("'{}'", &token).as_str());
    let stmt = client.prepare(&_stmt).await?;

    let queried_data = client
        .query(&stmt, &[])
        .await?
        .pop()
        .ok_or(Error::ColumnNotFound)?;

    User::from_row_ref(&queried_data)
}