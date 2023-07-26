/// Abstract base class for a Document retrieval system.
///
/// A retrieval system is defined as something that can take string queries and return
///     the most 'relevant' Documents from some source.
#[async_trait::async_trait]
pub trait BaseRetriever {}
