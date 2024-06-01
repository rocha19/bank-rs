use crate::{
    account_repository::AccountRepository, command::Command, observer::Observer,
    transfer_command::TransferCommand, transfer_service::TransferService,
};

pub struct TransferHandler {
    account_repository: Box<dyn AccountRepository>,
}

impl TransferHandler {
    pub fn new(account_repository: Box<dyn AccountRepository>) -> Self {
        TransferHandler { account_repository }
    }
}

impl Observer for TransferHandler {
    fn notify(&self, command: &dyn Command) {
        if let Some(transfer_command) = command.as_any().downcast_ref::<TransferCommand>() {
            let account_from = self
                .account_repository
                .get(&transfer_command.account_document_from);
            let account_to = self
                .account_repository
                .get(&transfer_command.account_document_to);

            let transfer_service = TransferService::new();

            if let Some(mut account_from) = account_from {
                if let Some(mut account_to) = account_to {
                    transfer_service.transfer(
                        &mut account_from,
                        &mut account_to,
                        transfer_command.amount,
                    );
                } else {
                    println!("Account to not found");
                }
            } else {
                println!("Account from not found");
            }
        }
    }

    fn operation(&self) -> &str {
        "transfer"
    }
}
