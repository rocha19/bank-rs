use crate::{
    account_repository::AccountRepository, command::Command, debit_command::DebitCommand,
    observer::Observer,
};

pub struct DebitHandler {
    account_repository: Box<dyn AccountRepository>,
}

impl DebitHandler {
    pub fn new(account_repository: Box<dyn AccountRepository>) -> Self {
        DebitHandler { account_repository }
    }
}

impl Observer for DebitHandler {
    fn notify(&self, command: &dyn Command) {
        if let Some(debit_command) = command.as_any().downcast_ref::<DebitCommand>() {
            if let Some(mut account) = self.account_repository.get(&debit_command.account_document)
            {
                account.debit(debit_command.amount);
                self.account_repository.save(account);
            } else {
                println!("Account not found");
            }
        }
    }

    fn operation(&self) -> &str {
        "debit"
    }
}
