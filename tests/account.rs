#[cfg(test)]
mod tests {
    use bank::{
        account_application_service::AccountApplicationService,
        account_repository_in_memory::AccountRepositoryMemory, publisher::Publisher,
    };

    #[test]
    fn test_create_account() {
        let publisher = Publisher::new();
        let account_repository = Box::new(AccountRepositoryMemory::new());
        let service = AccountApplicationService::new(publisher, account_repository);

        service.create("111.111.111-11".to_string());
        let account = service.get("111.111.111-11".to_string());
        assert_eq!(account.get_balance(), 0.0);
    }

    #[test]
    fn test_create_account_and_credit() {
        let publisher = Publisher::new();
        let account_repository = Box::new(AccountRepositoryMemory::new());
        let service = AccountApplicationService::new(publisher, account_repository);

        service.create("111.111.111-11".to_string());
        service.credit("111.111.111-11".to_string(), 1000.0);
        let account = service.get("111.111.111-11".to_string());
        assert_eq!(account.get_balance(), 1000.0);
    }

    #[test]
    fn test_create_account_and_debit() {
        let publisher = Publisher::new();
        let account_repository = Box::new(AccountRepositoryMemory::new());
        let service = AccountApplicationService::new(publisher, account_repository);

        service.create("111.111.111-11".to_string());
        service.credit("111.111.111-11".to_string(), 1000.0);
        service.debit("111.111.111-11".to_string(), 500.0);
        let account = service.get("111.111.111-11".to_string());
        assert_eq!(account.get_balance(), 500.0);
    }

    #[test]
    fn test_create_two_accounts_and_transfer() {
        let publisher = Publisher::new();
        let account_repository = Box::new(AccountRepositoryMemory::new());
        let service = AccountApplicationService::new(publisher, account_repository);

        service.create("111.111.111-11".to_string());
        service.credit("111.111.111-11".to_string(), 1000.0);
        service.create("222.222.222-22".to_string());
        service.credit("222.222.222-22".to_string(), 500.0);
        service.transfer(
            "111.111.111-11".to_string(),
            "222.222.222-22".to_string(),
            700.0,
        );
        let account_from = service.get("111.111.111-11".to_string());
        let account_to = service.get("222.222.222-22".to_string());
        assert_eq!(account_from.get_balance(), 300.0);
        assert_eq!(account_to.get_balance(), 1200.0);
    }
}
