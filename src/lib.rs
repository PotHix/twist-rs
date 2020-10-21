pub mod comments;
pub mod endpoints;
pub mod search;

const TWIST_URL: &'static str = "https://api.twist.com/";
const TWIST_API_URL: &'static str = "https://api.twist.com/api/v3";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
