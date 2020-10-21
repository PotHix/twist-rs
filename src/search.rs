use super::endpoints::search;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TwistSearch {
    pub has_more: bool,
    pub items: Vec<TwistSearchItem>,
}

#[derive(Serialize, Deserialize)]
pub struct TwistSearchItem {
    pub snippet: String,
    pub obj_type: String,
    pub details_link: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct TwistSearchDetails {
    pub highlights: Vec<String>,
    pub items: Vec<TwistSearchDetailsObj>,
}

#[derive(Serialize, Deserialize)]
pub struct TwistSearchDetailsObj {
    pub obj: Option<TwistSearchExpandedCommentItem>,
    pub expanded: Option<bool>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TwistSearchExpandedCommentItem {
    pub id: i32,
    pub content: String,
    pub thread_id: Option<i32>,
}

pub fn search(token: String, query: String) -> Result<TwistSearch, Box<dyn std::error::Error>> {
    let res = search::twist_search(token, query)?;
    let search: TwistSearch = serde_json::from_str(&res)?;
    return Ok(search);
}

#[tokio::main]
pub async fn details(
    token: String,
    details_link: String,
) -> Result<TwistSearchDetails, Box<dyn std::error::Error>> {
    let res = search::twist_search(token, details_link)?;
    let search: TwistSearchDetails = serde_json::from_str(&res)?;
    return Ok(search);
}
