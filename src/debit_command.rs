use crate::command::Command;

pub struct DebitCommand {
    operation: String,
    pub account_document: String,
    pub amount: f64,
}

impl DebitCommand {
    pub fn new(account_document: String, amount: f64) -> Self {
        DebitCommand {
            operation: "debit".to_string(),
            account_document,
            amount,
        }
    }
}

impl Command for DebitCommand {
    fn get_operation(&self) -> &str {
        &self.operation
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
