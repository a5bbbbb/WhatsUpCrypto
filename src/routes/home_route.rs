use axum::Router;

use tower_http::services::{
    ServeDir,
    ServeFile
};

pub fn home_routes() -> Router {
    Router::new()
    .route_service("/", ServeFile::new("src/views/index.html"))
    .nest_service("/assets", ServeDir::new("src/assets"))
}