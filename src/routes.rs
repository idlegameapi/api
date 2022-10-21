use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use warp::{Rejection, Reply};

use crate::{
    auth,
    errors::*,
    models::{User, UserWithoutSecrets},
};

/// Authorizes a user based on a Basic Auth header
pub async fn authorize(
    db_pool: deadpool_postgres::Pool,
    auth_header: String,
) -> Result<(deadpool_postgres::Pool, User), Rejection> {
    let auth::Auth { username, password } = auth::validate_header(auth_header.as_str())?;
    let pool = db_pool
        .get()
        .await
        .to_internal_error()?;

    let user = crate::db::get_user_by_username(&pool, &username)
        .await
        .map_err(|err| match err {
            tokio_pg_mapper::Error::ColumnNotFound => warp::reject::custom(NotFound),
            _ => warp::reject::custom(InternalError),
        })?;

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &user.salt)
        .to_internal_error()?
        .to_string();

    let parsed_hash = PasswordHash::new(&password_hash).to_internal_error()?;

    if argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        Ok((db_pool, user))
    } else {
        Err(warp::reject::custom(NotAuthorized))
    }
}

/// Create a new user via providing a Basic Auth header
pub async fn create_account(
    db_pool: deadpool_postgres::Pool,
    auth_header: String,
) -> Result<impl Reply, Rejection> {
    let auth::Auth { username, password } = auth::validate_header(auth_header.as_str())?;
    let pool = db_pool
        .get()
        .await
        .to_internal_error()?;

    let user = crate::db::get_user_by_username(&pool, &username)
        .await
        .map_err(|err| match err {
            tokio_pg_mapper::Error::ColumnNotFound => warp::reject::custom(NotFound),
            _ => warp::reject::custom(InternalError),
        });

    if let Ok(_) = user {
        return Err(warp::reject::custom(Conflict));
    }

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .to_internal_error()?
        .to_string();

    let new_user = crate::db::create_user(&pool, &username, &password_hash, &salt.to_string())
        .await
        .to_internal_error()?;

    Ok(warp::reply::json(&UserWithoutSecrets::from(new_user)))
}
