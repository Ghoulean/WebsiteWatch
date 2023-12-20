use std::env;

const SNS_TOPIC_ARN_KEY: &str = "SNS_TOPIC_ARN";

#[derive(Debug, PartialEq)]
pub struct EnvironmentConfig {
    pub sns_topic_arn: String
}

#[derive(Default)]
pub struct EnvironmentAccessor {}

impl EnvironmentAccessor {
    pub fn get_env(&self) -> EnvironmentConfig {
        let sns_topic_arn: String = env::var(SNS_TOPIC_ARN_KEY).expect("SNS topic arn not found");
        return EnvironmentConfig {
            sns_topic_arn: sns_topic_arn
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_test_env() {
        let under_test = EnvironmentAccessor {};
        
        let test_value: String = String::from("test");

        env::set_var(SNS_TOPIC_ARN_KEY, &test_value);

        let expected_environment_config: EnvironmentConfig = EnvironmentConfig {
            sns_topic_arn: test_value
        };
        assert_eq!(expected_environment_config, under_test.get_env());
    }
}