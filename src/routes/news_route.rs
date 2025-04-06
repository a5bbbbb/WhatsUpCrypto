use axum::routing::get;
use axum::Router;

use crate::handlers::news_handler::get_crypto_news;

pub fn news_routes() -> Router {
    Router::new().route("/news", get(get_crypto_news))
}

