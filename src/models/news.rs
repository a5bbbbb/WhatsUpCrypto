//title, source, date, summary, link

use serde::Deserialize;


#[derive(Deserialize,Debug)]
pub struct News{
    pub title:String,
    pub source:String,
    pub date:String,
    pub summary:String,
    pub link:String,
}
