use super::{account::Account, command::Command};

struct TransferCommand {
    operation: &'static str,
    account_from: Box<Account>,
    account_to: Box<Account>,
    amount: f64,
}

impl TransferCommand {
    pub fn new(
        account_from: Box<Account>,
        account_to: Box<Account>,
        amount: f64,
    ) -> TransferCommand {
        TransferCommand {
            operation: "transfer",
            account_from,
            account_to,
            amount,
        }
    }
}

impl Command for TransferCommand {
    fn operation(&self) -> &str {
        self.operation
    }
}
