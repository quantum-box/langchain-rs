use serde::{Deserialize, Serialize};
use std::fmt;

// The base message type. Messages are the inputs and outputs of ChatModels.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseMessage {
    /// The string contents of the message.
    pub content: String,

    /// Any additional information.
    pub additional_kwargs: Option<std::collections::HashMap<String, String>>,
}

// A Message from a human.
#[derive(Serialize, Deserialize, Debug)]
pub struct HumanMessage {
    pub base: BaseMessage,

    /// Whether this Message is being passed in to the model as part of an example conversation.
    pub example: bool,
}

// A Message from an AI.
#[derive(Serialize, Deserialize, Debug)]
pub struct AIMessage {
    pub base: BaseMessage,

    /// Whether this Message is being passed in to the model as part of an example conversation.
    pub example: bool,
}

// A Message for priming AI behavior, usually passed in as the first of a sequence of input messages.
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemMessage {
    pub base: BaseMessage,
}

// A Message for passing the result of executing a function back to a model.
#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionMessage {
    pub base: BaseMessage,

    /// The name of the function that was executed.
    pub name: String,
    pub arguments: Option<std::collections::HashMap<String, String>>,
}

// A Message that can be assigned an arbitrary speaker (i.e. role).
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub base: BaseMessage,

    /// The speaker / role of the Message.
    pub role: String,
}

// Enum to wrap all message types
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Human(HumanMessage),
    AI(AIMessage),
    System(SystemMessage),
    Function(FunctionMessage),
    Chat(ChatMessage),
}

// Implement Display trait for Message to control its string representation
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::Human(hm) => write!(f, "Human: {}", hm.base.content),
            Message::AI(am) => write!(f, "AI: {}", am.base.content),
            Message::System(sm) => write!(f, "System: {}", sm.base.content),
            Message::Function(fm) => write!(f, "Function: {}", fm.base.content),
            Message::Chat(cm) => write!(f, "{}: {}", cm.role, cm.base.content),
        }
    }
}

/// Convert sequence of Messages to strings and concatenate them into one string.
///
/// # Arguments
///
/// * `messages` - Messages to be converted to strings.
///
/// # Examples
///
/// ```rust
/// let messages = vec![
///     Message::Human(HumanMessage { base: BaseMessage { content: "Hi, how are you?".to_string(), additional_kwargs: None }, example: false }),
///     Message::AI(AIMessage { base: BaseMessage { content: "Good, how are you?".to_string(), additional_kwargs: None }, example: false }),
/// ];
///
/// assert_eq!(get_buffer_string(messages), "Human: Hi, how are you?\nAI: Good, how are you?");
/// ```
pub fn get_buffer_string(messages: Vec<Message>) -> String {
    let mut string_messages = Vec::new();
    for message in messages {
        string_messages.push(format!("{}", message));
    }
    string_messages.join("\n")
}
