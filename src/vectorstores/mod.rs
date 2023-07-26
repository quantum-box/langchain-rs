pub mod inmemory;

pub use inmemory::*;

use crate::embeddings::Embeddings;
use crate::schema::{Document, VectorMetadata};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait VectorStore<E>
where
    E: Embeddings + Clone + Send + Sync + 'static,
{
    fn embeddings(&self) -> Option<Arc<E>>;
    async fn similarity_search(
        &self,
        query: &str,
        k: Option<usize>,
    ) -> anyhow::Result<Vec<Document<VectorMetadata>>>;
    async fn add_document(&mut self, documents: Vec<Document>) -> anyhow::Result<Vec<String>>;
    async fn delete_document(&mut self, ids: Vec<String>) -> anyhow::Result<bool>;
}

pub struct VectorStoreRetriever<E, S>
where
    E: Embeddings + Clone + Send + Sync + 'static,
    S: VectorStore<E> + ?Sized + Send + Sync + 'static,
{
    vectorstore: Arc<S>,
    _embeddings: std::marker::PhantomData<E>,
}

impl<E, S> VectorStoreRetriever<E, S>
where
    E: Embeddings + Clone + Send + Sync + 'static,
    S: VectorStore<E> + Send + Sync + 'static,
{
    pub fn new(vectorstore: S) -> Self {
        Self {
            vectorstore: Arc::new(vectorstore),
            _embeddings: std::marker::PhantomData,
        }
    }
}
