use super::*;
use crate::schema::VectorMetadata;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct InMemoryVectorStore<E>
where
    E: Embeddings + Clone + Send + Sync + 'static,
{
    embeddings: Option<Arc<E>>,
    documents: HashMap<Uuid, Document<VectorMetadata>>,
}

impl<E> InMemoryVectorStore<E>
where
    E: Embeddings + Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        InMemoryVectorStore {
            embeddings: None,
            documents: HashMap::new(),
        }
    }

    pub async fn from_document(documents: Vec<Document>, embeddings: E) -> anyhow::Result<Self> {
        let mut store = InMemoryVectorStore {
            embeddings: Some(Arc::new(embeddings)),
            documents: HashMap::new(),
        };
        store.add_document(documents).await?;
        Ok(store)
    }

    fn cosine_similarity(&self, a: &Vec<f32>, b: &Vec<f32>) -> f32 {
        let dot_product = self.dot_product(&a, &b);
        let a_norm = self.dot_product(&a, &a).sqrt();
        let b_norm = self.dot_product(&b, &b).sqrt();
        dot_product / (a_norm * b_norm)
    }
    fn dot_product(&self, a: &Vec<f32>, b: &Vec<f32>) -> f32 {
        // TODO: これサイズが1536とわかっているので、配列にしたほうがいいかも
        a.iter().zip(b.iter()).map(|(a, b)| a * b).sum()
    }

    /// 最も類似度が高いdocumentを返す
    async fn sort_similarity_documents(
        &self,
        docs: Vec<Document<VectorMetadata>>,
        query: &str,
    ) -> anyhow::Result<Vec<Document<VectorMetadata>>> {
        let query_vector = self
            .embeddings
            .clone()
            .ok_or(anyhow::anyhow!("embeddings is None"))?
            .embed_query(query)
            .await?;

        let mut doc_vectors: Vec<(Document<VectorMetadata>, f32)> = Vec::new();

        for doc in docs {
            let doc_vector = doc
                .metadata
                .clone()
                .ok_or(anyhow::anyhow!("metadata is None"))?
                .0;
            let similarity = self.cosine_similarity(&query_vector, &doc_vector);
            doc_vectors.push((doc, similarity));
        }

        // Sort the documents by similarity in descending order
        doc_vectors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Extract the sorted documents
        let sorted_docs: Vec<Document<VectorMetadata>> =
            doc_vectors.into_iter().map(|(doc, _)| doc).collect();

        Ok(sorted_docs)
    }

    pub fn as_retriever(&self) -> VectorStoreRetriever<E, Self> {
        VectorStoreRetriever::new(self.clone())
    }
}

#[async_trait::async_trait]
impl<E> VectorStore<E> for InMemoryVectorStore<E>
where
    E: Embeddings + Clone + Send + Sync + 'static,
{
    fn embeddings(&self) -> Option<Arc<E>> {
        self.embeddings.clone()
    }

    async fn add_document(&mut self, documents: Vec<Document>) -> anyhow::Result<Vec<String>> {
        let mut ids = Vec::new();
        let vectors = self
            .embeddings
            .clone()
            .ok_or(anyhow::anyhow!("embeddings is None"))?
            .embed_document(&documents)
            .await?;
        for (index, document) in documents.iter().enumerate() {
            let document: Document<VectorMetadata> = Document {
                page_content: document.page_content.clone(),
                lookup_str: document.lookup_str.clone(),
                lookup_index: document.lookup_index,
                metadata: Some(vectors[index].clone().into()),
            };
            let id = Uuid::new_v4();
            self.documents.insert(id.clone(), document);
            ids.push(id);
        }
        Ok(ids.iter().map(|id| id.to_string()).collect())
    }

    async fn similarity_search(
        &self,
        query: &str,
        k: Option<usize>,
    ) -> anyhow::Result<Vec<Document<VectorMetadata>>> {
        let k = k.unwrap_or(4);
        let docs: Vec<Document<VectorMetadata>> = self.documents.values().cloned().collect();
        let docs = self.sort_similarity_documents(docs, query).await?;
        Ok(docs.into_iter().take(k).collect())
    }

    async fn delete_document(&mut self, ids: Vec<String>) -> anyhow::Result<bool> {
        let mut deleted = false;
        for id in ids {
            if let Ok(id) = Uuid::parse_str(&id) {
                if self.documents.remove(&id).is_some() {
                    deleted = true;
                }
            }
        }
        Ok(deleted)
    }
}
