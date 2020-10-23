pub mod comments;
pub mod endpoints;
pub mod search;

const URL: &'static str = "https://api.twist.com";
const API_VERSION: &'static str = "/api/v3";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
