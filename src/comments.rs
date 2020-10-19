use std::env;

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

pub fn add(thread_id: i32, content: String) -> Result<Comment, Box<std::error::Error>> {
    let suffix = "/api/v3/comments/add";

    if let Ok(token) = env::var("auth") {
        let params = [("content", content), ("thread_id", thread_id.to_string())];
        let bearer_token = "Bearer ".to_owned() + &token;

        let client = reqwest::Client::new();
        let mut res = client
            .post((super::TWIST_API.to_owned() + suffix).as_str())
            .header("Authorization", bearer_token)
            .form(&params)
            .send()?;

        let comment: Comment = res.json()?;
        return Ok(comment);
    } else {
        panic!("Token not available for the request");
    }
}
