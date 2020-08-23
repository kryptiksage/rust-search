extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_gitlab_url(query: &str) -> String {
    if query == "gl" {
        let gitlab_url = "https://gitlab.com";
        gitlab_url.to_string()
    }
    else {
        construct_gitlab_profile_url(&query[3..])
    }
}

pub fn construct_gitlab_profile_url(profile: &str) -> String {
    let encoded_query = utf8_percent_encode(&profile, FRAGMENT).to_string();
    format!("https://gitlab.com/{}", encoded_query)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construct_gitlab_url() {
        let fake_query = "gl";
        assert_eq!(construct_gitlab_url(fake_query), "https://gitlab.com");
    }
    #[test]
    fn test_construct_gitlab_url_query() {
        let fake_query = "gl kryptiksage";
        assert_eq!(construct_gitlab_url(fake_query), "https://gitlab.com/kryptiksage");
    }
    #[test]
    fn test_construct_gitlab_url_profile() {
        let fake_query = "gl kryptiksage/dotfiles";
        assert_eq!(construct_gitlab_url(fake_query), "https://gitlab.com/kryptiksage/dotfiles");
    }
}
