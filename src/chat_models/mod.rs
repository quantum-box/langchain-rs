pub mod openai;
pub use crate::schema::*;
use anyhow::Result;

#[async_trait::async_trait]
pub trait BaseChatModel {
    async fn _generate(&self, messages: Vec<Message>) -> Result<LLMResult>;
    async fn call(&self, messages: Vec<Message>) -> Result<BaseMessage> {
        let result = self._generate(messages).await?;
        Ok(BaseMessage {
            content: result.generations[0][0].text.clone(),
            additional_kwargs: None,
        })
    }
}
