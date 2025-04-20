use crate::{error::PushError, message::PushMessage};
use async_trait::async_trait;

#[async_trait]
pub trait PushClient: Send + Sync {
    async fn send(&self, message: PushMessage) -> Result<(), PushError>;

    fn name(&self) -> &'static str;
}
