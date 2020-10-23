#[cfg(test)]
use mockito;
use reqwest::blocking::Response;

pub fn search(token: String, query: String) -> Result<Response, Box<dyn std::error::Error>> {
    #[cfg(not(test))]
    let mut url = super::super::URL.to_string();
    #[cfg(test)]
    let mut url = mockito::server_url();

    url.push_str(super::super::API_VERSION);
    url.push_str("/search/query");

    let params = [("query", query)];
    let bearer_token = "Bearer ".to_string() + &token;

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url.as_str())
        .header("Authorization", bearer_token)
        .query(&params)
        .send()?;

    return Ok(res);
}

pub fn details(
    token: String,
    details_link: String,
) -> Result<Response, Box<dyn std::error::Error>> {
    let mut url = super::super::URL.to_string();
    url.push_str(details_link.as_str());

    let bearer_token = "Bearer ".to_string() + &token;

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url.as_str())
        .header("Authorization", bearer_token)
        .send()?;

    return Ok(res);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    #[ignore]
    fn request_a_search_query() {
        let _m = mock("GET", "/search/query")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{}")
            .create();

        if let Ok(res) = search("token".to_string(), "query".to_string()) {
            assert_eq!(res.text().unwrap(), "{}");
        }
    }
}
