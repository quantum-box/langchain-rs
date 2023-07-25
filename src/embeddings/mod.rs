pub mod openai;

pub use openai::*;

use crate::schema::Document;

#[async_trait::async_trait]
pub trait Embeddings: Send + Sync + 'static {
    async fn embed_document(&self, documents: &Vec<Document>) -> anyhow::Result<Vec<Vec<f32>>>;
    async fn embed_query(&self, text: &str) -> anyhow::Result<Vec<f32>>;
}
