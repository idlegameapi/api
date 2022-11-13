extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod auth;
pub mod config;
pub mod db;
pub mod errors;
pub mod models;
pub mod prelude;
pub mod routes;
pub mod utils;

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

    pretty_env_logger::init();

    // note(SirH): ✨ *database* config ✨
    let config = crate::config::Config::new();
    // note(SirH): this pool will go through routes so then you can interact with the db via this manager
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let is_alive = warp::path::end()
        .and(warp::get())
        .and_then(routes::is_alive);

    let auth_header = warp::any()
        .map(move || pool.clone())
        .and(warp::header::<String>("authorization"));

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

    let routes = claim
        .or(collect)
        .or(upgrade)
        .or(is_alive)
        .recover(errors::handle_rejection);

    // use 0.0.0.0 instead of 127.0.0.1,
    // due to silent host binding errors in some
    // hoster
    warp::serve(routes)
        .run(([0, 0, 0, 0], dotenv::var("PORT").unwrap().parse().unwrap()))
        .await;
}
