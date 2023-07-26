use super::*;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait BaseLanguageModel {
    /// Pass a sequence of prompts to the model and return model generations.
    ///
    /// This method should make use of batched calls for models that expose a batched
    /// API.
    ///
    /// Use this method when you want to:
    ///     1. take advantage of batched calls,
    ///     2. need more output from the model than just the top generated value,
    ///     3. are building chains that are agnostic to the underlying language model
    ///         type (e.g., pure text completion models vs chat models).
    ///
    /// # Arguments
    ///
    /// * `prompts` - List of PromptValues. A PromptValue is an object that can be
    ///     converted to match the format of any language model (string for pure
    ///     text generation models and BaseMessages for chat models).
    /// * `stop` - Stop words to use when generating. Model output is cut off at the
    ///     first occurrence of any of these substrings.
    /// * `callbacks` - Callbacks to pass through. Used for executing additional
    ///     functionality, such as logging or streaming, throughout generation.
    /// * `kwargs` - Arbitrary additional keyword arguments. These are usually passed
    ///     to the model provider API call.
    ///
    /// # Returns
    ///
    /// An LLMResult, which contains a list of candidate Generations for each input
    /// prompt and additional model provider-specific output.
    async fn generate_prompt(
        &self,
        prompts: Vec<Box<dyn PromptValue>>,
        stop: Option<Vec<String>>,
        kwargs: Option<std::collections::HashMap<String, String>>,
    ) -> Result<LLMResult>;

    /// Pass a single string input to the model and return a string prediction.
    ///
    /// Use this method when passing in raw text. If you want to pass in specific
    /// types of chat messages, use predict_messages.
    ///
    /// # Arguments
    ///
    /// * `text` - String input to pass to the model.
    /// * `stop` - Stop words to use when generating. Model output is cut off at the
    ///     first occurrence of any of these substrings.
    /// * `kwargs` - Arbitrary additional keyword arguments. These are usually passed
    ///     to the model provider API call.
    ///
    /// # Returns
    ///
    /// Top model prediction as a string.
    async fn predict(
        &self,
        text: String,
        stop: Option<Vec<String>>,
        kwargs: Option<std::collections::HashMap<String, String>>,
    ) -> Result<String>;

    /// Pass a message sequence to the model and return a message prediction.
    ///
    /// Use this method when passing in chat messages. If you want to pass in raw text,
    /// use predict.
    ///
    /// # Arguments
    ///
    /// * `messages` - A sequence of chat messages corresponding to a single model input.
    /// * `stop` - Stop words to use when generating. Model output is cut off at the
    ///     first occurrence of any of these substrings.
    /// * `kwargs` - Arbitrary additional keyword arguments. These are usually passed
    ///     to the model provider API call.
    ///
    /// # Returns
    ///
    /// Top model prediction as a message.
    async fn predict_messages(
        &self,
        messages: Vec<BaseMessage>,
        stop: Option<Vec<String>>,
        kwargs: Option<std::collections::HashMap<String, String>>,
    ) -> Result<BaseMessage>;

    // Remaining methods omitted for brevity
}
