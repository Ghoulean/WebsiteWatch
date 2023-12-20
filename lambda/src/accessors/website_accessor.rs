use reqwest;

#[derive(Default)]
pub struct WebsiteAccessor {}

impl WebsiteAccessor {
    pub async fn get_webpage(self: &Self, url: &String) -> Option<String>  {
        let response = reqwest::get(url).await.ok()?;
        if response.status().is_success() {
            let content: String = response.text().await.ok()?;
            return Some(content);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_webpage() {
        let under_test = WebsiteAccessor {};

        let test_url: String = String::from("http://example.com/");

        let page = under_test.get_webpage(&test_url).await;

        assert!(page.is_some());
        assert!(page.unwrap().contains("Example Domain"));
    }
}