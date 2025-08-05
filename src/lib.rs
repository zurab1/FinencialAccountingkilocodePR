//! Financial Accounting System
//! 
//! A comprehensive double-entry bookkeeping system built with Rust.

pub mod models;
pub mod database;
pub mod handlers;

pub use database::Database;
pub use models::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_account_creation() {
        let db = Database::new(":memory:").await.unwrap();
        
        let request = CreateAccountRequest {
            code: "9999".to_string(),
            name: "Test Account".to_string(),
            account_type: AccountType::Asset,
            parent_id: None,
        };
        
        let account = db.create_account(request).await.unwrap();
        
        assert_eq!(account.code, "9999");
        assert_eq!(account.name, "Test Account");
        assert_eq!(account.account_type, AccountType::Asset);
        assert_eq!(account.balance, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_transaction_validation() {
        let request = CreateTransactionRequest {
            description: "Test transaction".to_string(),
            reference: None,
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            journal_entries: vec![
                CreateJournalEntryRequest {
                    account_id: 1,
                    debit_amount: Some(Decimal::new(10000, 2)), // $100.00
                    credit_amount: None,
                    description: None,
                },
                CreateJournalEntryRequest {
                    account_id: 2,
                    debit_amount: None,
                    credit_amount: Some(Decimal::new(10000, 2)), // $100.00
                    description: None,
                },
            ],
        };
        
        // Should validate successfully
        assert!(request.validate().is_ok());
    }

    #[tokio::test]
    async fn test_unbalanced_transaction_validation() {
        let request = CreateTransactionRequest {
            description: "Unbalanced transaction".to_string(),
            reference: None,
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            journal_entries: vec![
                CreateJournalEntryRequest {
                    account_id: 1,
                    debit_amount: Some(Decimal::new(10000, 2)), // $100.00
                    credit_amount: None,
                    description: None,
                },
                CreateJournalEntryRequest {
                    account_id: 2,
                    debit_amount: None,
                    credit_amount: Some(Decimal::new(5000, 2)), // $50.00
                    description: None,
                },
            ],
        };
        
        // Should fail validation
        assert!(request.validate().is_err());
    }

    #[tokio::test]
    async fn test_account_normal_balance() {
        let asset_account = Account {
            id: 1,
            code: "1110".to_string(),
            name: "Cash".to_string(),
            account_type: AccountType::Asset,
            parent_id: None,
            balance: Decimal::new(10000, 2), // $100.00 debit balance
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        assert!(asset_account.has_normal_balance());
        assert_eq!(asset_account.normal_balance(), Decimal::new(10000, 2));
        
        let liability_account = Account {
            id: 2,
            code: "2110".to_string(),
            name: "Accounts Payable".to_string(),
            account_type: AccountType::Liability,
            parent_id: None,
            balance: Decimal::new(-10000, 2), // $100.00 credit balance
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        assert!(liability_account.has_normal_balance());
        assert_eq!(liability_account.normal_balance(), Decimal::new(10000, 2));
    }

    #[tokio::test]
    async fn test_account_summary() {
        let mut summary = AccountSummary::new();
        
        let cash_account = Account {
            id: 1,
            code: "1110".to_string(),
            name: "Cash".to_string(),
            account_type: AccountType::Asset,
            parent_id: None,
            balance: Decimal::new(50000, 2), // $500.00
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        let revenue_account = Account {
            id: 2,
            code: "4100".to_string(),
            name: "Sales Revenue".to_string(),
            account_type: AccountType::Revenue,
            parent_id: None,
            balance: Decimal::new(-50000, 2), // $500.00 credit
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        summary.add_account(&cash_account);
        summary.add_account(&revenue_account);
        
        assert_eq!(summary.total_assets, Decimal::new(50000, 2));
        assert_eq!(summary.total_revenue, Decimal::new(50000, 2));
        assert_eq!(summary.net_income, Decimal::new(50000, 2));
    }

    #[tokio::test]
    async fn test_trial_balance() {
        let mut trial_balance = TrialBalance::new();
        
        let entry = TrialBalanceEntry {
            account_id: 1,
            account_code: "1110".to_string(),
            account_name: "Cash".to_string(),
            account_type: AccountType::Asset,
            debit_balance: Decimal::new(10000, 2),
            credit_balance: Decimal::ZERO,
        };
        
        trial_balance.add_entry(entry);
        
        assert_eq!(trial_balance.total_debits, Decimal::new(10000, 2));
        assert_eq!(trial_balance.total_credits, Decimal::ZERO);
        assert!(!trial_balance.is_balanced);
    }
}pub mod app_state;
pub use app_state::AppState;
