mod accessors;
mod processing;

use crate::accessors::environment_accessor::EnvironmentAccessor;
use crate::accessors::sns_accessor::SnsAccessor;
use crate::accessors::website_accessor::WebsiteAccessor;
use crate::accessors::website_watch_config_accessor::WebsiteWatchConfigAccessor;
use crate::processing::executor::Executor;
use crate::processing::hash_generator::HashGenerator;

async fn handler(_event: lambda_runtime::LambdaEvent<serde_json::Value>) -> Result<(), lambda_runtime::Error> {
    let environment_accessor = EnvironmentAccessor::default();
    let environment_config = environment_accessor.get_env();

    let aws_config = aws_config::load_from_env().await;
    let sns_client = aws_sdk_sns::Client::new(&aws_config);

    let sns_accessor = SnsAccessor::new(sns_client, environment_config.sns_topic_arn);
    let website_accessor = WebsiteAccessor::default();
    
    let website_watch_config_accessor = WebsiteWatchConfigAccessor::default();
    let website_watch_configs = website_watch_config_accessor.get_website_watch_configs();
    println!("num configs: {}", &website_watch_configs.configs.len());

    let hash_generator = HashGenerator::default();

    let executor = Executor::new(sns_accessor, website_accessor, website_watch_configs, hash_generator);

    println!("executing...");
    executor.execute().await;
    println!("finished execution");
    return Ok(())
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(
        lambda_runtime::service_fn(handler)
    ).await
}
