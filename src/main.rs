mod routes;
mod handlers;
mod services;
mod models;
mod utils;


use axum::Router;
use routes::{home_route::home_routes, news_route::news_routes};

#[tokio::main]
async fn main() {

    let app = Router::new().merge(news_routes()).merge(home_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("Listening on localhost:5000");

    axum::serve(listener, app).await.unwrap();
}
