//title, source, date, summary, link
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Debug)]
pub struct News{
    pub title:String,
    pub source:String,
    pub date:String,
    pub summary:String,
    pub link:String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NewsApiResponse {
    #[serde(skip_serializing)]
    status: String,
    totalResults: i32,
    articles: Vec<Article>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    #[serde(skip_serializing)]
    source: Source,
    author: Option<String>,
    title: String,
    description: Option<String>,
    url: String,
    #[serde(skip_serializing)]
    urlToImage: Option<String>,
    publishedAt: String,
    content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    id: Option<String>,
    name: String,
}
