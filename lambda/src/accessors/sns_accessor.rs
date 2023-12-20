use aws_sdk_sns::error::DisplayErrorContext;

pub struct SnsAccessor {
    pub sns_client: aws_sdk_sns::Client,
    pub sns_topic_arn: String
}

impl SnsAccessor {
    pub fn new(
        sns_client: aws_sdk_sns::Client,
        sns_topic_arn: String
    ) -> Self {
        Self {
            sns_client: sns_client,
            sns_topic_arn: sns_topic_arn
        }
    }

    pub async fn send_sms_message(self: &Self, message: &String, phone_number: &String) -> Result<(), Box<dyn std::error::Error>>  {
        let response = self.sns_client.publish()
                .topic_arn(self.sns_topic_arn.as_str())
                .phone_number(phone_number)
                .message(message)
                .send()
                .await;
        
        if response.is_err() {
            let err = response.unwrap_err();
            eprintln!("sms unhandled error: {}", DisplayErrorContext(&err));
            return Err(Box::new(err));
        }

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn send_sms_message() {
        // TBD
    }
}