use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Generation {
    /// Generated text output.
    pub text: String,

    /// Raw response from the provider. May include things like the
    /// reason for finishing or token log probabilities.
    pub generation_info: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatGeneration {
    pub base: Generation,
    pub message: BaseMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunInfo {
    /// A unique identifier for the model or chain run.
    pub run_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResult {
    /// List of the chat generations. This is a List because an input can have multiple
    /// candidate generations.
    pub generations: Vec<ChatGeneration>,

    pub llm_output: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LLMResult {
    /// List of generated outputs. This is a List[List[]] because
    /// each input could have multiple candidate generations.
    pub generations: Vec<Vec<Generation>>,

    pub llm_output: Option<HashMap<String, String>>,

    pub run: Option<Vec<RunInfo>>,
}
