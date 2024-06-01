use crate::{account_builder::AccountBuilder, transaction::Transaction};

#[derive(Clone)]
pub struct Account {
    pub bank: Option<String>,
    pub branch: Option<String>,
    pub account: Option<String>,
    pub document: String,
    pub transactions: Vec<Transaction>,
}

impl Account {
    pub fn new(builder: AccountBuilder) -> Self {
        Account {
            bank: builder.bank,
            branch: builder.branch,
            account: builder.account,
            document: builder.document,
            transactions: Vec::new(),
        }
    }

    pub fn credit(&mut self, amount: f64) {
        self.transactions
            .push(Transaction::new(String::from("credit"), amount));
    }

    pub fn debit(&mut self, amount: f64) {
        self.transactions
            .push(Transaction::new(String::from("debit"), amount));
    }

    pub fn get_balance(&self) -> f64 {
        let mut balance = 0.0;
        for transaction in &self.transactions {
            match &transaction.transaction_type[..] {
                "credit" => balance += transaction.amount,
                "debit" => balance -= transaction.amount,
                _ => (),
            }
        }
        balance
    }
}
