use langchain::document_loaders::*;
use langchain::embeddings::*;
use langchain::text_splitter::*;
use langchain::vectorstores::*;

/// https://python.langchain.com/docs/use_cases/question_answering/#2-retrieval
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Document loader
    let loader = BrowserURLLoader::default("https://lilianweng.github.io/posts/2023-06-23-agent/");
    let data = loader.load().await?;

    // Split
    let text_splitter = RecursiveCharacterTextSplitter::new(500, 0, None);
    let all_splits = text_splitter.split_document(data);

    // Store
    let openai_embedding = OpenAIEmbedding::default();
    let store = InMemoryVectorStore::from_document(all_splits, openai_embedding).await?;

    // retrieval
    let question = "What are the approaches to Task Decomposition?";
    let docs = store.similarity_search(question, None).await?;
    println!("docs: {}", docs.len());

    // skip it for now
    // https://python.langchain.com/docs/use_cases/question_answering/#222-advanced-retrieval
    Ok(())
}
