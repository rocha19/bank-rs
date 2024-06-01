#[derive(Clone)]
pub struct Transaction {
    pub transaction_type: String,
    pub amount: f64,
}

impl Transaction {
    pub fn new(transaction_type: String, amount: f64) -> Self {
        Transaction {
            transaction_type,
            amount,
        }
    }
}
