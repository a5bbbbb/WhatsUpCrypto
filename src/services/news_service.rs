use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT};


use crate::{models::news::NewsApiResponse, utils};
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
    let page_size: String = "5".to_string();


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
