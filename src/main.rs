mod routes;
mod handlers;
mod services;
mod models;
mod utils;
mod middleware;


use axum::Router;
use routes::news_route::news_routes;
use middleware::rate_limit::rate_limit;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .merge(news_routes())
        .route_layer(axum::middleware::from_fn(rate_limit));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("Listening on localhost:5000");

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
