#[derive(Debug, Clone)]
pub struct Document<M = EmptyMetadata> {
    pub page_content: String,
    pub lookup_str: String,
    pub lookup_index: usize,
    pub metadata: Option<M>,
}

impl Document {
    pub fn new(page_content: &str, lookup_index: usize) -> Self {
        Self {
            page_content: page_content.to_string(),
            lookup_str: "".to_string(),
            lookup_index,
            metadata: None,
        }
    }
    pub fn paragraphs(&self) -> Vec<&str> {
        self.page_content.split("\n\n").collect()
    }

    pub fn summary(&self) -> Option<&str> {
        self.paragraphs().first().map(|p| *p)
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            page_content: "".to_string(),
            lookup_str: "".to_string(),
            lookup_index: 0,
            metadata: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmptyMetadata;

#[derive(Debug, Clone)]
pub struct VectorMetadata(pub Vec<f32>);
