use crate::domain::entities::{account_builder::AccountBuilder, credit_command::CreditCommand};

use super::account_repository::AccountRepository;

pub struct AccountService<'a> {
    account_repository: &'a mut dyn AccountRepository,
    publisher: Publisher,
}

impl AccountService<'_> {
    pub fn new(
        publisher: Publisher,
        account_repository: &mut dyn AccountRepository,
    ) -> AccountService {
        AccountService {
            publisher,
            account_repository,
        }
    }

    pub fn create(&mut self, document: &str) {
        let account = AccountBuilder::new(document).build();
        self.account_repository.save(account);
    }

    pub fn credit(&self, account_document: &str, amount: f64) {
        let credit_command = CreditCommand::new(account_document, amount);
        self.publisher.publish(credit_command);
    }
}
