use super::command::Command;

#[allow(dead_code)]
pub struct CreditCommand {
    pub operation: &'static str,
    pub account_document: String,
    pub amount: f64,
}

impl CreditCommand {
    pub fn new(account_document: &str, amount: f64) -> CreditCommand {
        CreditCommand {
            operation: "credit",
            account_document: account_document.to_string(),
            amount,
        }
    }
}

impl Command for CreditCommand {
    fn operation(&self) -> &str {
        self.operation
    }
}
