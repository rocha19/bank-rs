use super::{account_builder::AccountBuilder, transaction::Transaction};

#[derive(Clone)]
pub struct Account {
    pub bank: Option<String>,
    pub branch: Option<String>,
    pub account: Option<String>,
    pub document: String,
    pub transactions: Vec<Transaction>,
}

impl Account {
    pub fn new(account_builder: AccountBuilder) -> Account {
        Account {
            bank: account_builder.bank,
            branch: account_builder.branch,
            account: account_builder.account,
            document: account_builder.document,
            transactions: vec![],
        }
    }

    pub fn credit(&mut self, amount: f64) {
        self.transactions
            .push(Transaction::new("credit".to_string(), amount));
    }

    pub fn debit(&mut self, amount: f64) {
        self.transactions
            .push(Transaction::new("debit".to_string(), amount));
    }

    pub fn get_balance(&self) -> f64 {
        let mut balance = 0.0;

        for transaction in &self.transactions {
            if transaction.operation == "credit" {
                balance += transaction.amount;
            }

            if transaction.operation == "debit" {
                balance -= transaction.amount;
            }
        }

        balance
    }
}
