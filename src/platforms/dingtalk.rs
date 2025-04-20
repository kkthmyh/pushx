use crate::{client::PushClient, error::PushError, message::PushMessage};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct DingTalkClient {
    pub webhook: String,
    pub secret: Option<String>, 
    client: Client,
}

impl DingTalkClient {
    pub fn new(webhook: String, secret: Option<String>) -> Self {
        Self {
            webhook,
            secret,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl PushClient for DingTalkClient {
    async fn send(&self, message: PushMessage) -> Result<(), PushError> {
        let body = json!({
            "msgtype": "text",
            "text": {
                "content": format!("{}\n{}", message.title, message.content)
            }
        });

        let resp = self.client.post(&self.webhook).json(&body).send().await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(PushError::UnexpectedResponse(resp.status().to_string()))
        }
    }

    fn name(&self) -> &'static str {
        "dingtalk"
    }
}
