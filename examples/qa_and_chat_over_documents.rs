use langchain::document_loaders::*;
use langchain::text_splitter::*;

/// https://python.langchain.com/docs/use_cases/question_answering/#2-retrieval
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Document loader
    let loader = BrowserURLLoader::default("https://lilianweng.github.io/posts/2023-06-23-agent/");
    let data = loader.load().await?;

    // Split
    let text_splitter = RecursiveCharacterTextSplitter::new(500, 0, None);
    let _all_splits = text_splitter.split_document(data);

    // Store
    Ok(())
}
