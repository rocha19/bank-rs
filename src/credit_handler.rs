use crate::{
    account_repository::AccountRepository, command::Command, credit_command::CreditCommand,
    observer::Observer,
};

pub struct CreditHandler {
    account_repository: Box<dyn AccountRepository>,
}

impl CreditHandler {
    pub fn new(account_repository: Box<dyn AccountRepository>) -> Self {
        CreditHandler { account_repository }
    }
}

impl Observer for CreditHandler {
    fn notify(&self, command: &dyn Command) {
        if let Some(credit_command) = command.as_any().downcast_ref::<CreditCommand>() {
            if let Some(mut account) = self
                .account_repository
                .get(&credit_command.account_document)
            {
                account.credit(credit_command.amount);
                self.account_repository.save(account);
            } else {
                println!("Account not found");
            }
        }
    }

    fn operation(&self) -> &str {
        "credit"
    }
}
