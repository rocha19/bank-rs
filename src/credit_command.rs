use crate::command::Command;

pub struct CreditCommand {
    operation: String,
    pub account_document: String,
    pub amount: f64,
}

impl CreditCommand {
    pub fn new(account_document: String, amount: f64) -> Self {
        CreditCommand {
            operation: "credit".to_string(),
            account_document,
            amount,
        }
    }
}

impl Command for CreditCommand {
    fn get_operation(&self) -> &str {
        &self.operation
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
