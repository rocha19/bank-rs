use super::account::Account;

pub struct TransferService;

impl TransferService {
    pub fn transfer(&self, account_from: &mut Account, account_to: &mut Account, amount: f64) {
        account_from.debit(amount);
        account_to.credit(amount);
    }
}
