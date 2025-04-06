use axum::{Json, response::IntoResponse};
use crate::services::news_service;
use serde_json::json;

pub async fn get_crypto_news() -> impl IntoResponse {
    match news_service::fetch_data().await {
        Ok(news) => Json(json!(news)),
        Err(err) => {
            println!("Error during fetch: {}",err);
            return Json(json!({"error": "Failed to fetch news:"}));
        }
    }
}

