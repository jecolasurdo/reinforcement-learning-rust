#[derive(Debug, Clone, PartialEq)]
pub struct LearnerError {
    msg: String,
}

impl<'a> LearnerError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    pub fn message(&self) -> String {
        self.msg.clone()
    }
}
