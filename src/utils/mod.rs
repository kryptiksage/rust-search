pub mod google;
pub mod twitter;
pub mod gitlab;
pub mod github;

pub fn get_cmd_from_query(query: &str) -> &str {
    if query.contains(' ') {
        let index_of_space = query.find(' ').unwrap_or(0);
        return &query[..index_of_space]
    }
    query
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_cmd_from_query_no_space() {
        let actual = get_cmd_from_query("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_cmd_from_query_space() {
        let actual = get_cmd_from_query("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}
