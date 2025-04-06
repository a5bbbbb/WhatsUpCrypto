use crate::{models::news::NewsApiResponse, utils};
use reqwest::Error;
use crate::utils::utils::get_date_week_ago;


pub async fn fetch_data(req: &String) -> Result<NewsApiResponse, Error> {
    let news_api_key: Option<String> = utils::utils::get_news_api_key()
        .into_iter()
        .next()
        .flatten();


    let base_url: &str = "https://newsapi.org/v2/everything";
    let from: String = get_date_week_ago();
    let sort_by: &str = "popularity";
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
