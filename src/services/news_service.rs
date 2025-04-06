use crate::{models::news::NewsApiResponse, utils};
use reqwest::Error;


pub async fn fetch_data() -> Result<NewsApiResponse, Error> {
    let news_api_key: Option<String> = utils::utils::get_news_api_key()
        .into_iter()
        .next()
        .flatten();


    let base_url: &str = "https://newsapi.org/v2/everything";
    let from: &str = "2025-03-06";
    let sort_by: &str = "publishedAt";
    let req: &str = "tesla";
    let complete_url = match news_api_key {
        Some(val) => format!(
            "{}{}{}{}{}{}{}{}{}",
            base_url, "?q=", req, "&from=", from, "&sortBy=", sort_by, "&apiKey=", val
        ),

        None => base_url.to_string(),
    };

    let response = reqwest::get(&complete_url).await?;
    
    let news: NewsApiResponse = response.json().await?;

    Ok(news)

}
