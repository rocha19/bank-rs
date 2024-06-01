use crate::account::Account;

#[derive(Default)]
pub struct TransferService;

impl TransferService {
    pub fn new() -> Self {
        TransferService
    }

    pub fn transfer(&self, from: &mut Account, to: &mut Account, amount: f64) {
        from.debit(amount);
        to.credit(amount);
    }
}
