pub mod auth;
pub mod config;
pub mod db;
pub mod models;
pub mod routes;
pub mod errors;

use deadpool_postgres::Runtime;
use tokio_postgres::NoTls;
use warp::Filter;

#[macro_export]
macro_rules! warp_reply {
    ($x:expr, $y:ident) => {
        ::warp::reply::with_status($x, warp::http::StatusCode::$y)
    };
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // note(SirH): ✨ config ✨
    let config = crate::config::Config::new();
    // note(SirH): this pool will go through routes so then you can interact with the db via this manager
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let auth_header = warp::any().map(move || pool.clone()).and(warp::header::<String>("authorization"));

    let claim = warp::path("claim")
        .and(warp::post())
        .and(auth_header.clone())
        .and_then(routes::create_account);

    let collect = warp::path("collect")
        .and(warp::patch())
        .and(auth_header.clone())
        .and_then(routes::authorize)
        .and_then(routes::collect);

    let upgrade = warp::path("upgrade")
        .and(warp::patch())
        .and(auth_header.clone())
        .and_then(routes::authorize)
        .and_then(routes::upgrade);

    let routes = claim.or(collect).or(upgrade).recover(errors::handle_rejection);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
