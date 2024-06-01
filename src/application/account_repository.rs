use crate::domain::entities::account::Account;

pub trait AccountRepository {
    fn save(&mut self, account: Account);
    fn get(&self, document: &str) -> Option<Account>;
}
