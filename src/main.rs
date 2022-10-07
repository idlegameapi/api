pub mod auth;
pub mod config;
pub mod db;
pub mod models;
pub mod routes;

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
    let state_filter = warp::any().map(move || pool.clone());

    let routes = warp::any()
        .and(state_filter.clone())
        .and(warp::header::<String>("Authorization"))
        .and_then(routes::auth)
        .recover(routes::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
