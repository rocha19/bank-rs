use crate::command::Command;

pub struct TransferCommand {
    operation: String,
    pub account_document_from: String,
    pub account_document_to: String,
    pub amount: f64,
}

impl TransferCommand {
    pub fn new(account_document_from: String, account_document_to: String, amount: f64) -> Self {
        TransferCommand {
            operation: "transfer".to_string(),
            account_document_from,
            account_document_to,
            amount,
        }
    }
}

impl Command for TransferCommand {
    fn get_operation(&self) -> &str {
        &self.operation
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
