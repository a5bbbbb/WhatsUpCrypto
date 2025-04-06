use std::env;
use dotenv::dotenv;

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
