use serde::{Serialize, Deserialize};

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

#[tokio::main]
pub async fn search(token: String, query: String) -> Result<TwistSearch, Box<dyn std::error::Error>> {
    let suffix = "/search/query";

    let params = [("query", query)];
    let bearer_token = "Bearer ".to_owned() + &token;

    let client = reqwest::Client::new();
    let res = client
        .get((super::TWIST_API_URL.to_owned() + suffix).as_str())
        .header("Authorization", bearer_token)
        .query(&params)
        .send()
        .await?;

    let search: TwistSearch = res.json().await?;
    return Ok(search);
}

#[tokio::main]
pub async fn details(token: String, details_link: String) -> Result<TwistSearchDetails, Box<dyn std::error::Error>> {
    let bearer_token = "Bearer ".to_owned() + &token;

    let client = reqwest::Client::new();
    let res = client
        .get((super::TWIST_URL.to_owned() + details_link.as_str()).as_str())
        .header("Authorization", bearer_token)
        .send()
        .await?;

    let search: TwistSearchDetails = res.json().await?;
    return Ok(search);
}
