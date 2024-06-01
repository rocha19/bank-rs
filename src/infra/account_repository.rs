use crate::{
    application::account_repository::AccountRepository, domain::entities::account::Account,
};

pub struct AccountRepositoryInMemory {
    pub accounts: Vec<Account>,
}

impl AccountRepositoryInMemory {
    pub fn new() -> AccountRepositoryInMemory {
        AccountRepositoryInMemory { accounts: vec![] }
    }
}

impl AccountRepository for AccountRepositoryInMemory {
    fn save(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn get(&self, document: &str) -> Option<Account> {
        self.accounts
            .iter()
            .find(|account| account.document == document)
            .cloned()
    }
}
