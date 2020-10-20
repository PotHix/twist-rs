use serde_json;

pub fn search(token: String, query: String) -> String {
    let content = twist_rs::search::search(token, query);
    return serde_json::to_string(content)
}
