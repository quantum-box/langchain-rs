use crate::schema::{Document, EmptyMetadata};

pub trait TextSplitter<M: Clone = EmptyMetadata>
where
    Document<M>: Default,
    Vec<std::option::Option<M>>: FromIterator<std::option::Option<EmptyMetadata>>,
{
    fn split_text(&self, text: &str) -> Vec<String>;
    fn split_document(&self, documents: Vec<Document>) -> Vec<Document<M>> {
        let texts = documents
            .iter()
            .map(|doc| doc.page_content.clone())
            .collect();
        let metadatas = documents.iter().map(|doc| doc.metadata.clone()).collect();
        self.create_documents(texts, metadatas)
    }
    fn create_documents(&self, texts: Vec<String>, metadatas: Vec<Option<M>>) -> Vec<Document<M>> {
        let mut documents = Vec::new();
        for (i, text) in texts.into_iter().enumerate() {
            let metadata = &metadatas[i];
            for chunk in self.split_text(&text) {
                documents.push(Document {
                    page_content: chunk,
                    metadata: metadata.clone(),
                    ..Default::default()
                })
            }
        }
        documents
    }
    fn join_docs(&self, docs: Vec<String>, separator: &str) -> Option<String> {
        let text = docs.join(separator).trim().to_string();
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}

/// # RecursiveCharacterTextSplitter
///
/// https://github.com/hwchase17/langchain/blob/c2d1d903fa35b91018b4d777db2b008fcbaa9fbc/langchain/text_splitter.py#L221
#[derive(Debug, Clone)]
pub struct RecursiveCharacterTextSplitter {
    chunk_size: usize,
    chunk_overlap: usize,
    separators: Vec<String>,
}

impl RecursiveCharacterTextSplitter {
    pub fn new(chunk_size: usize, chunk_overlap: usize, separators: Option<Vec<String>>) -> Self {
        let separators =
            separators.unwrap_or_else(|| vec!["\n\n".into(), "\n".into(), " ".into(), "".into()]);
        Self {
            chunk_size,
            chunk_overlap,
            separators,
        }
    }

    fn merge_splits(&self, splits: Vec<String>, separator: &str) -> Vec<String> {
        let mut docs = Vec::new();
        let mut current_doc = Vec::new();
        let mut total = 0;
        for d in &splits {
            let len = d.len();
            if total + len >= self.chunk_size {
                // Rustでは組み込みのロギング機能がないので、logクレート等を使用する
                // if total > self.chunk_size {}
                if !current_doc.clone().is_empty() {
                    if let Some(doc) = self.join_docs(current_doc.clone(), separator) {
                        docs.push(doc);
                    }
                    while total > self.chunk_overlap || (total + len > self.chunk_size && total > 0)
                    {
                        total -= &current_doc[0].len();
                        current_doc.remove(0);
                    }
                }
            }
            current_doc.push(d.clone());
            total += len;
        }
        if let Some(doc) = self.join_docs(current_doc, separator) {
            docs.push(doc);
        }
        docs
    }
}

impl Default for RecursiveCharacterTextSplitter {
    fn default() -> Self {
        Self {
            chunk_size: 4000,
            chunk_overlap: 200,
            separators: vec![
                "\n".to_string(),
                "\n\n".to_string(),
                " ".to_string(),
                "".to_string(),
            ],
        }
    }
}

impl TextSplitter for RecursiveCharacterTextSplitter {
    fn split_text(&self, text: &str) -> Vec<String> {
        let mut final_chunks = Vec::new();
        let mut separator = self.separators.last().unwrap();
        for s in &self.separators {
            if s.is_empty() || text.contains(s) {
                separator = s;
                break;
            }
        }
        let splits = if !separator.is_empty() {
            text.split(separator).map(|s| s.into()).collect()
        } else {
            Vec::from_iter(text.chars().map(|c| c.to_string()))
        };
        let mut good_splits = Vec::new();
        for s in splits {
            if s.len() < self.chunk_size {
                good_splits.push(s);
            } else {
                if !good_splits.is_empty() {
                    final_chunks.extend(self.merge_splits(good_splits, separator));
                    good_splits = Vec::new();
                }
                final_chunks.extend(self.split_text(&s));
            }
        }
        if !good_splits.is_empty() {
            final_chunks.extend(self.merge_splits(good_splits, separator));
        }
        final_chunks
    }
}
