use crate::application::account_repository::AccountRepository;

use super::{command::Command, credit_command::CreditCommand, observer::Observer};

pub struct CreditHandler {
    operation: &'static str,
    account_repository: Box<dyn AccountRepository>,
}

impl CreditHandler {
    pub fn new(account_repository: Box<dyn AccountRepository>) -> CreditHandler {
        CreditHandler {
            operation: "credit",
            account_repository,
        }
    }
}

impl Observer for CreditHandler {
    fn notify(&self, command: Box<dyn Command>) {
        if let Some(credit_command) = command.as_any().downcast_ref::<CreditCommand>() {
            let account = self
                .account_repository
                .get(&credit_command.account_document);
            match account {
                Some(mut account) => {
                    account.credit(credit_command.amount);
                }
                None => {}
            }
        }
    }

    fn operation(&self) -> &'static str {
        self.operation
    }
}

pub trait Any {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> Any for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
