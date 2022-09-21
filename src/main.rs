mod db;
pub mod config;
pub mod models;
pub mod schema;
pub mod show_users;

use db::establish_connection;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    establish_connection();

    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}