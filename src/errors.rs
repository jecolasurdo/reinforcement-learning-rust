#[derive(Debug, Clone)]
pub struct LearnerError {
    msg: String,
}

impl<'a> LearnerError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
