use super::account::Account;

pub struct AccountBuilder {
    pub bank: Option<String>,
    pub branch: Option<String>,
    pub account: Option<String>,
    pub document: String,
}

impl AccountBuilder {
    pub fn new(document: &str) -> AccountBuilder {
        Self {
            bank: None,
            branch: None,
            account: None,
            document: document.to_string(),
        }
    }

    pub fn set_bank(mut self, bank: &str) -> AccountBuilder {
        self.bank = Some(bank.to_string());
        self
    }

    pub fn set_branch(mut self, branch: &str) -> AccountBuilder {
        self.branch = Some(branch.to_string());
        self
    }

    pub fn set_account(mut self, account: &str) -> AccountBuilder {
        self.account = Some(account.to_string());
        self
    }

    pub fn build(self) -> Account {
        Account {
            bank: self.bank,
            branch: self.branch,
            account: self.account,
            document: self.document,
            transactions: vec![],
        }
    }
}
