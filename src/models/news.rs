use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsApiResponse {
    pub status: String,
    pub totalResults: i32,
    pub articles: Vec<Article>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    pub source: Source,
    pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub urlToImage: Option<String>,
    pub publishedAt: String,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

impl NewsApiResponse {
    pub fn new(status: String, total_results: i32, articles: Vec<Article>) -> Self {
        Self {
            status,
            totalResults: total_results,
            articles,
        }
    }
}