pub mod db;
pub mod config;
pub mod models;
pub mod routes;
pub mod auth;

use warp::Filter;
use deadpool_postgres::Runtime;
use tokio_postgres::NoTls;

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
        .and(warp::header::<auth::AuthPair>("authorization"))
        .and_then(routes::auth);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}