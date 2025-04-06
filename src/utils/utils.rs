use std::{collections::HashMap, env, time::{SystemTime, UNIX_EPOCH}};
use dotenv::dotenv;
use lazy_static::lazy_static;
use chrono::{Utc, Duration};


lazy_static! {
    static ref CRYPTO_MAP: HashMap<String, String> = {
        let mut map = HashMap::new();
        
        map.insert("btc".to_string(), "Bitcoin".to_string());
        map.insert("bitcoin".to_string(), "Bitcoin".to_string());
        map.insert("₿".to_string(), "Bitcoin".to_string());
        
        map.insert("eth".to_string(), "Ethereum".to_string());
        map.insert("ethereum".to_string(), "Ethereum".to_string());
        map.insert("ether".to_string(), "Ethereum".to_string());
        
        map.insert("xrp".to_string(), "Ripple".to_string());
        map.insert("ripple".to_string(), "Ripple".to_string());
        
        map
    };
}

const NEWS_API: &str = "newsAPI";

pub fn get_news_api_key() -> Vec<Option<String>> {
    dotenv().ok();
    
    let mut vec: Vec<Option<String>> = Vec::new();

    match env::var(NEWS_API) {
        Ok(val) => vec.push(Some(val)),
        Err(e) => {
            println!("Error loading env var: {:?}", e);
            vec.push(None);
        },
    }
    
    return vec;
}

pub fn get_date_week_ago() -> String {
    let today = Utc::now();
    let week_ago = today - Duration::days(7);
    week_ago.format("%Y-%m-%d").to_string()
}


pub fn sanitize_coin_input(coin_input:&str) -> Option<&String> {
    return CRYPTO_MAP.get(&coin_input.to_lowercase());
}


pub fn get_current_time_millis() -> u128{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Internal server error")
        .as_millis()
}
