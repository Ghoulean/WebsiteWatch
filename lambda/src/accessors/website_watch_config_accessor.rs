use serde::Deserialize;

const CONFIG_FILE_CONTENTS: &str = include_str!("../res/config.json");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebsiteWatchConfigs {
    pub configs: Vec<WebsiteWatchConfig>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebsiteWatchConfig {
    pub webpage_url: String,
    pub webpage_hash: String,
    pub phone_number: String // E.164 format
}

#[derive(Default)]
pub struct WebsiteWatchConfigAccessor {}

impl WebsiteWatchConfigAccessor {
    pub fn get_website_watch_configs(self: &Self) -> WebsiteWatchConfigs  {
        let config_file_contents = CONFIG_FILE_CONTENTS;
        let configs: WebsiteWatchConfigs = serde_json::from_str(config_file_contents).expect("JSON was not well-formatted");
        return configs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // fragile test that relies on real config.json
    #[test]
    fn get_website_watch_configs() {
        let under_test = WebsiteWatchConfigAccessor {};
        let configs = under_test.get_website_watch_configs();

        assert_eq!(1, configs.configs.len());
    }
}