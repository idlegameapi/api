use sha2::{Digest, Sha256};
use warp::{Rejection, Reply};

use crate::auth;

#[derive(Debug)]
struct InternalError;

impl warp::reject::Reject for InternalError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(crate::warp_reply!(
            "There is no user with that name".to_owned(),
            NOT_FOUND
        ))
    } else if let Some(e) = err.find::<auth::AuthorizationError>() {
        Ok(crate::warp_reply!(format!("{:?}", e), BAD_REQUEST))
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
) -> std::result::Result<impl Reply, Rejection> {
    let auth::Auth { username, password } = auth::validate_header(auth_header.as_str()).await?;
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
    let mut x = Vec::from(password.as_bytes());
    x.extend(user.salt.as_bytes());

    hasher.update(x);
    let result = hasher.finalize();

    todo!();

    Ok(crate::warp_reply!("Ok", OK))
}
