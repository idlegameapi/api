use sha2::{Digest, Sha256};
use warp::{Rejection, Reply};

use crate::auth;

#[derive(Debug)]
struct InternalError;

impl warp::reject::Reject for InternalError {}

#[derive(Debug)]
struct NotAuthorized;

impl warp::reject::Reject for NotAuthorized {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(crate::warp_reply!(
            "There is no user with that name".to_owned(),
            NOT_FOUND
        ))
    } else if let Some(e) = err.find::<auth::AuthorizationError>() {
        Ok(crate::warp_reply!(format!("{:?}", e), BAD_REQUEST))
    } else if err.find::<NotAuthorized>().is_some() {
        Ok(crate::warp_reply!(
            "The password is incorrect".to_string(),
            UNAUTHORIZED
        ))
    } else {
        eprintln!("An unknown error occured: {:?}", err);
        Ok(crate::warp_reply!(
            "Something went wrong, please try again later.".to_owned(),
            INTERNAL_SERVER_ERROR
        ))
    }
}

pub async fn auth(
    db_pool: deadpool_postgres::Pool,
    auth_header: String,
) -> std::result::Result<deadpool_postgres::Pool, Rejection> {
    let auth::Auth { username, password } = auth::validate_header(auth_header.as_str())?;
    let pool = db_pool
        .get()
        .await
        .map_err(|_| warp::reject::custom(InternalError))?;

    let user = crate::db::get_user(&pool, &username)
        .await
        .map_err(|err| match err {
            tokio_pg_mapper::Error::ColumnNotFound => warp::reject::not_found(),
            _ => warp::reject::custom(InternalError),
        })?;

    let mut hasher = Sha256::new();
    let mut x = password.into_bytes();
    x.extend(user.salt.trim().as_bytes());

    hasher.update(x);
    let result = hasher.finalize();

    if result[..] == user.token {
        Ok(db_pool)
    } else {
        Err(warp::reject::custom(NotAuthorized))
    }
}

pub async fn hello_world(
    db_pool: deadpool_postgres::Pool,
) -> Result<impl Reply, std::convert::Infallible> {
    Ok("Hello, authorized user!")
}
