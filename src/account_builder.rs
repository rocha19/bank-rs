use crate::account::Account;

pub struct AccountBuilder {
    pub bank: Option<String>,
    pub branch: Option<String>,
    pub account: Option<String>,
    pub document: String,
}

impl AccountBuilder {
    pub fn new(document: String) -> Self {
        AccountBuilder {
            bank: None,
            branch: None,
            account: None,
            document,
        }
    }

    pub fn set_bank(mut self, bank: String) -> Self {
        self.bank = Some(bank);
        self
    }

    pub fn set_branch(mut self, branch: String) -> Self {
        self.branch = Some(branch);
        self
    }

    pub fn set_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self
    }

    pub fn build(self) -> Account {
        Account::new(self)
    }
}
