use crate::account::Account;

pub trait AccountRepository {
    fn save(&self, account: Account);
    fn get(&self, account_document: &str) -> Option<Account>;
}
