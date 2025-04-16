use serde::Deserialize;
use crate::models::news::{Article, NewsApiResponse, Source};

#[derive(Debug, Deserialize)]
pub struct CoinDeskResponse {
    #[serde(rename = "Data")]
    pub data: Vec<CoinDeskArticle>,
    #[serde(rename = "Err")]
    pub err: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct CoinDeskArticle {
    #[serde(rename = "TYPE")]
    pub article_type: String,
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "GUID")]
    pub guid: Option<String>,
    #[serde(rename = "PUBLISHED_ON")]
    pub published_on: i64,
    #[serde(rename = "IMAGE_URL")]
    pub image_url: String,
    #[serde(rename = "TITLE")]
    pub title: String,
    #[serde(rename = "SUBTITLE")]
    pub subtitle: Option<String>,
    #[serde(rename = "AUTHORS")]
    pub authors: Option<String>,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "SOURCE_ID")]
    pub source_id: i64,
    #[serde(rename = "BODY")]
    pub body: String,
    #[serde(rename = "KEYWORDS")]
    pub keywords: Option<String>,
    #[serde(rename = "SOURCE_DATA")]
    pub source_data: CoinDeskSourceData,
    #[serde(rename = "CATEGORY_DATA")]
    pub category_data: Option<Vec<CoinDeskCategoryData>>,
}

#[derive(Debug, Deserialize)]
pub struct CoinDeskSourceData {
    #[serde(rename = "TYPE")]
    pub source_type: String,
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "SOURCE_KEY")]
    pub source_key: Option<String>,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "IMAGE_URL")]
    pub image_url: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "LANG")]
    pub lang: Option<String>,
    #[serde(rename = "SOURCE_TYPE")]
    pub source_type_str: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CoinDeskCategoryData {
    #[serde(rename = "TYPE")]
    pub category_type: String,
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "CATEGORY")]
    pub category: String,
}

pub fn adapt_coindesk_to_news_api(coindesk_response: CoinDeskResponse) -> NewsApiResponse {
    let articles: Vec<Article> = coindesk_response.data.into_iter()
        .map(|cd_article| {
            let timestamp = match chrono::DateTime::from_timestamp(cd_article.published_on, 0) {
                Some(ts) => ts,
                None => chrono::Utc::now(),
            };
            let iso_date = timestamp.to_rfc3339();
            
            let description = cd_article.subtitle.or_else(|| {
                Some(if cd_article.body.chars().count() > 150 {
                    format!("{}...", cd_article.body.chars().take(150).collect::<String>())
                } else {
                    cd_article.body.clone()
                })
            });            
            
            let content = Some(
                if cd_article.body.chars().count() > 300 {
                    format!("{}...", cd_article.body.chars().take(300).collect::<String>())
                } else {
                    cd_article.body.clone()
                }
            );
            
            
            Article {
                source: Source {
                    id: Some(cd_article.source_id.to_string()),
                    name: cd_article.source_data.name.clone(),
                },
                author: cd_article.authors.clone(),
                title: cd_article.title,
                description,
                url: cd_article.url,
                urlToImage: Some(cd_article.image_url),
                publishedAt: iso_date,
                content,
            }
        })
        .collect();
    
    NewsApiResponse::new("ok".to_string(), articles.len() as i32, articles)
}