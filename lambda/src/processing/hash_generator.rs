use regex::Regex;
use sha256::digest;

const HTML_CONTENT_REGEX_PATTERN: &str = r"(?ms)^(<!DOCTYPE html>)[\s\S]*(<\/html>)$";

#[derive(Default)]
pub struct HashGenerator {}

impl HashGenerator {
    pub fn hash_webpage_contents(self: &Self, webpage_response: &String) -> Option<String>  {
        let html_contents = self.extract_html_contents(webpage_response)?;
        return Some(digest(html_contents));
    }

    // current implementation is a quick hack
    fn extract_html_contents(self: &Self, raw_html_contents: &String) -> Option<String> {
        let regex = Regex::new(HTML_CONTENT_REGEX_PATTERN).ok()?;
        return Some(regex.find(raw_html_contents)?.as_str().to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_webpage_contents() {
        let under_test = HashGenerator {};

        let webpage_contents: String = String::from("
<!DOCTYPE html>
<html lang=\"en\">
<body></body>
</html>
<!--
    generated 284 seconds ago
    generated in 0.162 seconds
    served from batcache in 0.003 seconds
    expires in 16 seconds
-->");
        let expected_webpage_hash = String::from("373d99a74753aab9b6faecf57f15962804b7ee046680e53aebdf34e7de54eada");

        let actual_hash = under_test.hash_webpage_contents(&webpage_contents);
        assert!(actual_hash.is_some());
        assert_eq!(expected_webpage_hash, actual_hash.unwrap());
    }
}