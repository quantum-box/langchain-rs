use super::Embeddings;
use crate::schema::Document;
use async_openai::{config::OpenAIConfig, types::CreateEmbeddingRequestArgs, Client};

#[derive(Debug, Clone)]
pub struct OpenAIEmbedding {
    pub model: String,
    pub client: Client<OpenAIConfig>,
}

impl<'a> OpenAIEmbedding {
    pub fn new(model: &'a str) -> Self {
        Self {
            model: model.to_string(),
            client: Client::new(),
        }
    }
}

impl Default for OpenAIEmbedding {
    fn default() -> Self {
        Self {
            model: "text-embedding-ada-002".to_string(),
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl Embeddings for OpenAIEmbedding {
    async fn embed_document(&self, documents: &Vec<Document>) -> anyhow::Result<Vec<Vec<f32>>> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(&self.model)
            .input(
                documents
                    .iter()
                    .map(|doc| doc.page_content.clone())
                    .collect::<Vec<_>>(),
            )
            .build()?;
        let response = self.client.embeddings().create(request).await?;
        Ok(response.data.iter().map(|d| d.embedding.clone()).collect())
    }

    async fn embed_query(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(&self.model)
            .input(vec![text.to_string()])
            .build()?;
        let response = self.client.embeddings().create(request).await?;
        Ok(response.data[0].embedding.clone())
    }
}
