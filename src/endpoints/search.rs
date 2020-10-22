#[tokio::main]
pub async fn search(token: String, query: String) -> Result<String, Box<dyn std::error::Error>> {
    let suffix = "/search/query";

    let params = [("query", query)];
    let bearer_token = "Bearer ".to_owned() + &token;

    let client = reqwest::Client::new();
    let res = client
        .get((super::super::TWIST_API_URL.to_owned() + suffix).as_str())
        .header("Authorization", bearer_token)
        .query(&params)
        .send()
        .await?;

    return Ok(res.text().await?);
}

#[tokio::main]
pub async fn details(
    token: String,
    details_link: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let bearer_token = "Bearer ".to_owned() + &token;

    let client = reqwest::Client::new();
    let res = client
        .get((super::super::TWIST_URL.to_owned() + details_link.as_str()).as_str())
        .header("Authorization", bearer_token)
        .send()
        .await?;

    return Ok(res.text().await?);
}
