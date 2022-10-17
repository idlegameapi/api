use sha2::{Digest, Sha256};
use warp::{Rejection, Reply};

use crate::{auth, errors::*, models::User};

pub async fn auth(
    db_pool: deadpool_postgres::Pool,
    auth_header: String,
) -> Result<(deadpool_postgres::Pool, User), Rejection> {
    let auth::Auth { username, password } = auth::validate_header(auth_header.as_str())?;
    let pool = db_pool
        .get()
        .await
        .map_err(|_| warp::reject::custom(InternalError))?;

    let user = crate::db::get_user(&pool, &username)
        .await
        .map_err(|err| match err {
            tokio_pg_mapper::Error::ColumnNotFound => warp::reject::custom(NotFound),
            _ => warp::reject::custom(InternalError),
        })?;

    let mut hasher = Sha256::new();
    let mut x = password.into_bytes();
    x.extend(user.salt.trim().as_bytes());

    hasher.update(x);
    let result = hasher.finalize();

    if result[..] == user.token {
        Ok((db_pool, user))
    } else {
        Err(warp::reject::custom(NotAuthorized))
    }
}

pub async fn create_account(
    db_pool: deadpool_postgres::Pool,
    auth_header: String,
) -> Result<impl Reply, Rejection> {
    let auth::Auth { username, password } = auth::validate_header(auth_header.as_str())?;
    let pool = db_pool
        .get()
        .await
        .map_err(|_| warp::reject::custom(InternalError))?;

    let user = crate::db::get_user(&pool, &username)
        .await
        .map_err(|err| match err {
            tokio_pg_mapper::Error::ColumnNotFound => warp::reject::custom(NotFound),
            _ => warp::reject::custom(InternalError),
        });

    if let Ok(_) = user {
        return Err(warp::reject::custom(Conflict));
    }

    let salt: String = (0..10).map(|_| rand::random::<char>()).collect();

    let mut hasher = Sha256::new();
    let mut x = password.into_bytes();
    x.extend(salt.trim().as_bytes());

    hasher.update(x);
    let result = &hasher.finalize()[..];

    let new_user = crate::db::create_user(&pool, &username, result, &salt)
        .await
        .map_err(|_| warp::reject::custom(InternalError))?;

    Ok(crate::warp_reply!(format!("Welcome, {}!", new_user.username), CREATED))
}

pub async fn get_account(
    (_, user): (deadpool_postgres::Pool, User),
) -> Result<impl Reply, Rejection> {
    Ok(crate::warp_reply!(format!("Welcome back, {}!", user.username), OK))
}
