use super::*;
use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, FunctionCall, Role},
    Client,
};

#[derive(Debug)]
pub struct ChatOpenAI {
    client: Client<OpenAIConfig>,
    model: String,
    temperature: f32,
    openai_api_key: Option<String>,
}

impl ChatOpenAI {
    pub fn new(model: String, temperature: f32, openai_api_key: Option<String>) -> Result<Self> {
        let client = Client::new();
        Ok(Self {
            client,
            model,
            temperature,
            openai_api_key,
        })
    }
}

impl Default for ChatOpenAI {
    fn default() -> Self {
        let client = Client::new();
        Self {
            client,
            model: "gpt-3.5-turbo".to_string(),
            temperature: 0.7,
            openai_api_key: None,
        }
    }
}

#[async_trait::async_trait]
impl BaseChatModel for ChatOpenAI {
    async fn _generate(&self, messages: Vec<Message>) -> Result<LLMResult> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.clone())
            .messages(
                messages
                    .iter()
                    .map(|value| match value {
                        Message::System(text) => ChatCompletionRequestMessage {
                            role: Role::System,
                            content: Some(text.base.content.clone()),
                            name: None,
                            function_call: None,
                        },
                        Message::Human(text) => ChatCompletionRequestMessage {
                            role: Role::User,
                            content: Some(text.base.content.clone()),
                            name: None,
                            function_call: None,
                        },
                        Message::AI(text) => ChatCompletionRequestMessage {
                            role: Role::Assistant,
                            content: Some(text.base.content.clone()),
                            name: None,
                            function_call: None,
                        },
                        _ => panic!("Invalid message type"),
                    })
                    .collect::<Vec<ChatCompletionRequestMessage>>(),
            )
            .build()?;
        let response = self.client.chat().create(request).await?;
        Ok(LLMResult {
            generations: vec![vec![Generation {
                text: response.choices[0].message.content.clone().unwrap(),
                generation_info: None,
            }]],
            llm_output: None,
            run: None,
        })
    }
}

#[async_trait::async_trait]
impl BaseLanguageModel for ChatOpenAI {
    async fn generate_prompt(
        &self,
        prompts: Vec<Box<dyn PromptValue>>,
        stop: Option<Vec<String>>,
        kwargs: Option<std::collections::HashMap<String, String>>,
    ) -> Result<LLMResult> {
        unimplemented!()
    }
    async fn predict(
        &self,
        text: String,
        stop: Option<Vec<String>>,
        kwargs: Option<std::collections::HashMap<String, String>>,
    ) -> Result<String> {
        unimplemented!()
    }
    async fn predict_messages(
        &self,
        messages: Vec<BaseMessage>,
        stop: Option<Vec<String>>,
        kwargs: Option<std::collections::HashMap<String, String>>,
    ) -> Result<BaseMessage> {
        unimplemented!()
    }
}

impl From<Message> for ChatCompletionRequestMessage {
    fn from(value: Message) -> Self {
        match value {
            Message::System(text) => ChatCompletionRequestMessage {
                role: Role::System,
                content: Some(text.base.content),
                name: None,
                function_call: None,
            },
            Message::Human(text) => ChatCompletionRequestMessage {
                role: Role::User,
                content: Some(text.base.content),
                name: None,
                function_call: None,
            },
            Message::AI(text) => ChatCompletionRequestMessage {
                role: Role::Assistant,
                content: Some(text.base.content),
                name: None,
                function_call: None,
            },
            _ => panic!("Invalid message type"),
        }
    }
}
