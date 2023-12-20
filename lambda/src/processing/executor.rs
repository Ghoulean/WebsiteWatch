use crate::accessors::sns_accessor::SnsAccessor;
use crate::accessors::website_accessor::WebsiteAccessor;
use crate::accessors::website_watch_config_accessor::WebsiteWatchConfigs;
use crate::processing::hash_generator::HashGenerator;

pub struct Executor {
    pub sns_accessor: SnsAccessor,
    pub website_accessor: WebsiteAccessor,
    pub website_watch_configs: WebsiteWatchConfigs,
    pub hash_generator: HashGenerator
}

impl Executor {
    pub fn new(
            sns_accessor: SnsAccessor,
            website_accessor: WebsiteAccessor,
            website_watch_configs: WebsiteWatchConfigs,
            hash_generator: HashGenerator
        ) -> Self {
            Self {
                sns_accessor: sns_accessor,
                website_accessor: website_accessor,
                website_watch_configs: website_watch_configs,
                hash_generator: hash_generator
            }
    }

    pub async fn execute(self: &Self) {
        for config in self.website_watch_configs.configs.iter() {
            println!("getting webpage for {}", config.webpage_url);
            let webpage: Option<String> = self.website_accessor.get_webpage(&config.webpage_url).await;
            if webpage.is_none() {
                println!("can't get webpage for {}", &config.webpage_url);
                continue;
            }
            let hash: Option<String> = self.hash_generator.hash_webpage_contents(&webpage.unwrap());
            if hash.is_none() {
                println!("couldn't calculate hash for {}", &config.webpage_url);
                continue;
            }
            let unwrapped_hash: String = hash.unwrap();
            println!("obtained hash: {}", unwrapped_hash);
            if unwrapped_hash == config.webpage_hash {
                println!("hash matches for {}", &config.webpage_url);
                continue;
            }
            println!("hash: {} does not match expected: {}", unwrapped_hash, config.webpage_hash);
            let notif_message: String = format!(
                "detected hash diff in {}. expected: {:6}; actual; {:6}",
                config.webpage_url,
                config.webpage_hash,
                unwrapped_hash
            );
            println!("sending notif...");
            let res: Result<(), Box<dyn std::error::Error>> = self.sns_accessor.send_sms_message(&notif_message, &config.phone_number).await;
            if res.is_ok() {
                println!("successfully sent notification");
            } else {
                eprintln!("err sending sms notification: {}", res.unwrap_err());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn execute() {
        // TODO
    }
}