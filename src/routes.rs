use std::time::SystemTime;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use warp::{Rejection, Reply};

use crate::{
    auth,
    errors::*,
    models::{GameCalculations, User, UserWithoutSecrets},
};

/// Authorizes a user based on a Basic Auth header
pub async fn authorize(
    db_pool: deadpool_postgres::Pool,
    auth_header: String,
) -> Result<(deadpool_postgres::Pool, User), Rejection> {
    let auth::Auth { username, password } = auth::validate_header(auth_header.as_str())?;
    let pool = db_pool.get().await.to_internal_error()?;

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
    let pool = db_pool.get().await.to_internal_error()?;

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

/// Collect money for a user by multiplying their production by the time since last collection and then adding it to their balance
pub async fn collect(
    (db_pool, user): (deadpool_postgres::Pool, User),
) -> Result<impl Reply, Rejection> {
    let pool = db_pool.get().await.to_internal_error()?;

    let new_balance = user.balance
        + user.get_production()
            * user
                .collected_timestamp
                .duration_since(SystemTime::now())
                .to_internal_error()?
                .as_secs_f64();

    let updated_user = crate::db::collect_user(&pool, &user.username, new_balance)
        .await
        .to_internal_error()?;

    Ok(warp::reply::json(&updated_user))
}

/// Upgrade the user's levels
///
/// This will return a 403 if the user does not have enough money to upgrade
pub async fn upgrade(
    (db_pool,
    user): (deadpool_postgres::Pool, User),
) -> Result<impl Reply, Rejection> {
    let pool = db_pool.get().await.to_internal_error()?;

    let upgrade = user.upgradeable_levels();

    let upgrade = match upgrade {
        Ok(upgrade) => upgrade,
        Err(_) => return Err(warp::reject::custom(NotEnoughMoney)),
    };

    let updated_user =
        crate::db::upgrade_user(&pool, &user.username, user.balance - upgrade.cost, upgrade.level)
            .await
            .to_internal_error()?;

    Ok(warp::reply::json(&updated_user))
}
