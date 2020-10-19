pub mod comments;
pub mod search;

const TWIST_API: &'static str = "https://api.twist.com";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
