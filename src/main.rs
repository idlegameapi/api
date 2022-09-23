pub mod db;
pub mod config;
pub mod models;

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

    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
