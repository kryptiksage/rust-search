pub fn construct_google_search_url(query: &str) -> String {
    String::from("https://google.com/search?q=test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let fake_query = "hello";
        assert_eq!(construct_google_search_url(fake_query), "https://google.com/search?q=hello");
    }
    #[test]
    fn test_construct_google_search_url_with_encoding() {
        let fake_query = "hello world";
        assert_eq!(construct_google_search_url(fake_query), "https://google.com/search?q=hello%20world");
    }
}
