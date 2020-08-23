extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_github_url(query: &str) -> String {
    if query == "gh" {
        let github_url = "https://github.com";
        github_url.to_string()
    }
    else {
        construct_github_profile_url(&query[3..])
    }
}

pub fn construct_github_profile_url(profile: &str) -> String {
    let encoded_query = utf8_percent_encode(&profile, FRAGMENT).to_string();
    format!("https://github.com/{}", encoded_query)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construct_github_url() {
        let fake_query = "gh";
        assert_eq!(construct_github_url(fake_query), "https://github.com");
    }
    #[test]
    fn test_construct_github_url_query() {
        let fake_query = "gh kryptiksage";
        assert_eq!(construct_github_url(fake_query), "https://github.com/kryptiksage");
    }
    #[test]
    fn test_construct_github_url_profile() {
        let fake_query = "gh kryptiksage/dotfiles";
        assert_eq!(construct_github_url(fake_query), "https://github.com/kryptiksage/dotfiles");
    }
}
