pub mod browser;
pub mod slack;

pub use browser::*;
pub use slack::*;

use crate::schema::Document;
use crate::text_splitter::{RecursiveCharacterTextSplitter, TextSplitter};

#[async_trait::async_trait]
pub trait Loader<S: TextSplitter + Default + Send + Sync + 'static = RecursiveCharacterTextSplitter>
{
    async fn load(&self) -> anyhow::Result<Vec<Document>>;
    async fn load_and_spin(&self, text_splitter: Option<S>) -> anyhow::Result<Vec<Document>> {
        let text_splitter: S = if let Some(text_splitter) = text_splitter {
            text_splitter
        } else {
            S::default()
        };
        let docs = self.load().await?;
        Ok(text_splitter.split_document(docs))
    }
}
