use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT};


use crate::{models::news::NewsApiResponse, utils};
use crate::models::coindesk_adapter::{CoinDeskResponse, adapt_coindesk_to_news_api};
use crate::utils::utils::get_date_week_ago;


pub async fn fetch_data(req: &String) -> Result<NewsApiResponse, Box<dyn std::error::Error>> {

    let news_api_key: Option<String> = utils::utils::get_news_api_key()
        .into_iter()
        .next()
        .flatten();

    let sort_by: String = "popularity".to_string();

    let api_key = match news_api_key {
        Some(key) => key,
        None => return Err("Failed to get API_KEY".into()),
    };
    let from = get_date_week_ago();
    let page_size: String = "30".to_string();


    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml"));


    let client = reqwest::Client::new();
    let response = client
        .get("https://newsapi.org/v2/everything/")
        .headers(headers)
        .query(&[
            ("q", req),
            ("from", &from),
            ("sortBy", &sort_by),
            ("pageSize",&page_size),
            ("apiKey", &api_key),
        ])
        .send()
        .await?;

    let news: NewsApiResponse = response.json().await?;
    println!("Successfully retrieved news");
    
    Ok(news)
}

pub async fn fetch_coindesk_data(category: &str, limit: u32) -> Result<NewsApiResponse, Box<dyn std::error::Error>>{
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let response = client
        .get("https://data-api.coindesk.com/news/v1/article/list")
        .headers(headers)
        .query(&[
            ("lang", "EN"),
            ("limit", &limit.to_string()),
            ("categories", category),
        ])
        .send()
        .await?;

        let coindesk_data: CoinDeskResponse = response.json().await?;
        println!("Successfully retrieved CoinDesk news");

        Ok(adapt_coindesk_to_news_api(coindesk_data))
}

pub async fn fetch_combined_news(req: &String) -> Result<NewsApiResponse, Box<dyn std::error::Error>> {
    let category = if req.to_lowercase().contains("ethereum") || req.to_uppercase().contains("ETH") {
        "ETH"
    } else if req.to_lowercase().contains("bitcoin") || req.to_uppercase().contains("BTC") {
        "BTC"
    } else {
        "ETH" 
    };
    
    let news_api_result = fetch_data(req).await?;
    println!("NewsAPI returned {} articles", news_api_result.articles.len());

    let coindesk_result = match fetch_coindesk_data(category, 30).await {
        Ok(news) => {
            println!("CoinDesk API returned {} articles", news.articles.len());
            news
        },
        Err(e) => {
            println!("Error fetching from CoinDesk: {}", e);
            return Ok(news_api_result);
        }
    };
    
    let mut combined_articles = Vec::new();
    combined_articles.extend(news_api_result.articles.clone());
    combined_articles.extend(coindesk_result.articles.clone());
    
    combined_articles.sort_by(|a, b| {
        b.publishedAt.cmp(&a.publishedAt)
    });
    
    Ok(NewsApiResponse::new(
        "ok".to_string(),
        combined_articles.len() as i32,
        combined_articles
    ))
}