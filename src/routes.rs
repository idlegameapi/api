use warp::{Rejection, Reply};

use crate::auth;

pub async fn auth(
    db_pool: deadpool_postgres::Pool,
    auth_header: auth::Auth,
) -> std::result::Result<impl Reply, Rejection> {
    todo!()
}
