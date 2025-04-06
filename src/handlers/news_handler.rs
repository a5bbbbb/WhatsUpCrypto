use std::collections::HashMap;

use axum::{extract::Query, response::IntoResponse, Json};
use crate::services::news_service;
use crate::utils::utils::sanitize_coin_input;
use serde_json::json;

const COIN_KEY: &str = "coin";

pub async fn get_crypto_news(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {

    let coin:&str = match params.get(COIN_KEY) {
        Some(val) => val,
        None => return Json(json!({"error": "You did not pass any coin!"})),

    };

    let sanitized_coin:&String = match sanitize_coin_input(&coin) {
        Some(val) => val,
        None => return Json(json!({"error": "You passed unknown symbol!"})),
    }; 

    match news_service::fetch_data(sanitized_coin).await {
        Ok(news) => Json(json!(news)),
        Err(err) => {
            println!("Error during fetch: {}",err);
            return Json(json!({"error": format!("Failed to fetch news: {}", err)}));
        }
    }
}


