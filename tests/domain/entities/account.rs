// use chrono::NaiveDateTime;
// NaiveDateTime::parse_from_str("2024-05-29T16:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

#[cfg(test)]
mod account_test {
    use bank::{
        application::account_service::AccountService,
        domain::entities::{
            account_builder::AccountBuilder, command::Command, credit_command::CreditCommand,
            debit_command::DebitCommand, transfer_command::TransferCommand,
        },
        infra::account_repository::AccountRepositoryInMemory,
    };

    // INFO: Should be able to create an account;
    #[tokio::test]
    async fn create_account() {
        let account = AccountBuilder::new("990.909.999-09")
            .set_bank("033")
            .set_branch("001")
            .set_account("98757-9")
            .build();

        assert_eq!(account.get_balance(), 0.0);
    }

    // INFO: Should be able to create an account and make a credit;
    #[tokio::test]
    async fn create_account_and_credit() {
        // let mut account = AccountBuilder::new("990.909.999-09")
        //     .set_bank("033")
        //     .set_branch("001")
        //     .set_account("98757-9")
        //     .build();

        let mut account_repository = AccountRepositoryInMemory::new();
        let account_service = AccountService::new(&account_repository);
        account_service.create("990.909.999-09");
        let mut credit_command = CreditCommand::new(&mut account, 1000.0);

        credit_command.perform();
        assert_eq!(account.get_balance(), 1000.0);
    }

    // INFO: Should be able to create an account and make a debit;
    #[tokio::test]
    async fn create_account_and_debit() {
        let mut account = AccountBuilder::new("990.909.999-09")
            .set_bank("033")
            .set_branch("001")
            .set_account("98757-9")
            .build();

        let mut credit_command = CreditCommand::new(&mut account, 1000.0);
        credit_command.perform();

        let mut debit_command = DebitCommand::new(&mut account, 500.0);
        debit_command.perform();
        assert_eq!(account.get_balance(), 500.0);
    }

    // INFO: Should be create twe accounts and make a transaction;
    #[tokio::test]
    async fn create_accounts_and_transaction() {
        let mut account_from = AccountBuilder::new("999.999.999-99")
            .set_bank("033")
            .set_branch("001")
            .set_account("987654-3")
            .build();

        let mut account_to = AccountBuilder::new("000.000.000-00")
            .set_bank("033")
            .set_branch("001")
            .set_account("123456-7")
            .build();

        let mut credit_command_from = CreditCommand::new(&mut account_from, 1000.0);
        credit_command_from.perform();

        let mut credit_command_to = CreditCommand::new(&mut account_to, 500.0);
        credit_command_to.perform();

        let mut transfer_command = TransferCommand::new(&mut account_from, &mut account_to, 700.0);
        transfer_command.perform();
        assert_eq!(account_from.get_balance(), 300.0);
        assert_eq!(account_to.get_balance(), 1200.0);
    }
}
