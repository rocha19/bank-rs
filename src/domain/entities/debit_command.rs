use super::command::Command;

#[allow(dead_code)]
pub struct DebitCommand {
    operation: &'static str,
    account_document: String,
    amount: f64,
}

impl DebitCommand {
    pub fn new(account_document: String, amount: f64) -> DebitCommand {
        DebitCommand {
            operation: "debit",
            account_document,
            amount,
        }
    }
}

impl Command for DebitCommand {
    fn operation(&self) -> &str {
        self.operation
    }
}
