#[derive(Clone)]
pub struct Transaction {
    pub operation: String,
    pub amount: f64,
}

impl Transaction {
    pub fn new(operation: String, amount: f64) -> Transaction {
        Transaction { operation, amount }
    }
}
