use super::*;

pub trait PromptValue: Send {
    /// Return prompt value as string.
    fn to_string(&self) -> String;

    /// Return prompt as a list of Messages.
    fn to_messages(&self) -> Vec<BaseMessage>;
}
