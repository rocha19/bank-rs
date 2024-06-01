use crate::{
    account::Account, account_builder::AccountBuilder, account_repository::AccountRepository,
    credit_command::CreditCommand, debit_command::DebitCommand, publisher::Publisher,
    transfer_command::TransferCommand,
};

pub struct AccountApplicationService {
    publisher: Publisher,
    account_repository: Box<dyn AccountRepository>,
}

impl AccountApplicationService {
    pub fn new(publisher: Publisher, account_repository: Box<dyn AccountRepository>) -> Self {
        AccountApplicationService {
            publisher,
            account_repository,
        }
    }

    pub fn create(&self, document: String) {
        let account = AccountBuilder::new(document).build();
        self.account_repository.save(account);
    }

    pub fn credit(&self, account_document: String, amount: f64) {
        let credit_command = CreditCommand::new(account_document, amount);
        self.publisher.publish(&credit_command);
    }

    pub fn debit(&self, account_document: String, amount: f64) {
        let debit_command = DebitCommand::new(account_document, amount);
        self.publisher.publish(&debit_command);
    }

    pub fn transfer(
        &self,
        account_document_from: String,
        account_document_to: String,
        amount: f64,
    ) {
        let transfer_command =
            TransferCommand::new(account_document_from, account_document_to, amount);
        self.publisher.publish(&transfer_command);
    }

    pub fn get(&self, account_document: String) -> Account {
        if let Some(account) = self.account_repository.get(&account_document) {
            account
        } else {
            panic!("Account not found");
        }
    }
}
