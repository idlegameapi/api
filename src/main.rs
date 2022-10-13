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
    let account_filter = warp::path("account")
        .and(state_filter.clone())
        .and(warp::header::<String>("Authorization"));

    // note(SirH): I know there are multiple things wrong with this but their chaining of ands, thens, and ors doesn't want to work in the way I want it to
    // so I'm leaving it as is for now
    let account_creation = warp::post().and_then(routes::create_account);

    let account_retrieval = warp::get()
        .and(account_filter.clone())
        .and_then(routes::auth)
        .and_then(routes::hello_world)
        .recover(routes::handle_rejection);

    let routes = account_creation.or(account_retrieval);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
