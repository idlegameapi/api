use warp::{Reply, Rejection};

use crate::auth;

pub async fn auth(db_pool: deadpool_postgres::Pool, auth_header: auth::AuthPair) -> std::result::Result<impl Reply, Rejection> {
    todo!()
}