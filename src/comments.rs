use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub workspace_id: i32,
    pub thread_id: i32,
    pub deleted: bool,
    pub creator: i32,
    pub content: String,
    pub channel_id: i32,
    pub id: i32,
}

#[tokio::main]
pub async fn add(token: String, thread_id: i32, content: String) -> Result<Comment, Box<dyn std::error::Error>> {
    let suffix = "/comments/add";

    let params = [("content", content), ("thread_id", thread_id.to_string())];
    let bearer_token = "Bearer ".to_owned() + &token;

    let client = reqwest::Client::new();
    let res = client
        .post((super::TWIST_API_URL.to_owned() + suffix).as_str())
        .header("Authorization", bearer_token)
        .form(&params)
        .send()
        .await?;

    let comment: Comment = res.json().await?;
    return Ok(comment);
}
