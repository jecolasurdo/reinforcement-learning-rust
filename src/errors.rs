//! Error types associated with the reinforcement learning process.

#[derive(Debug, Clone, PartialEq)]
/// A general error that has occurred during a learning operation.
pub struct LearnerError {
    msg: String,
}

impl<'a> LearnerError {
    /// Instantiates a new `LearnerError` with a message.
    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    /// A message associated with this error.
    pub fn message(&self) -> String {
        self.msg.clone()
    }
}
