use std::cell::RefCell;

use crate::{account::Account, account_repository::AccountRepository};

#[derive(Default)]
pub struct AccountRepositoryMemory {
    accounts: RefCell<Vec<Account>>,
}

impl AccountRepositoryMemory {
    pub fn new() -> Self {
        AccountRepositoryMemory {
            accounts: RefCell::new(Vec::new()),
        }
    }
}

impl AccountRepository for AccountRepositoryMemory {
    fn save(&self, account: Account) {
        self.accounts.borrow_mut().push(account);
    }

    fn get(&self, account_document: &str) -> Option<Account> {
        self.accounts
            .borrow()
            .iter()
            .find(|&acc| acc.document == account_document)
            .cloned()
    }
}
