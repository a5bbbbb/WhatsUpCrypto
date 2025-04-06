mod routes;
mod handlers;
mod services;
mod models;
mod utils;


use axum::Router;
use routes::news_route::news_routes;

#[tokio::main]
async fn main() {

    let app = Router::new().merge(news_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
