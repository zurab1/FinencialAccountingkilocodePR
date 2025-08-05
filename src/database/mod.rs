use sqlx::{SqlitePool, Row};
use anyhow::Result;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};
use std::str::FromStr;

use crate::models::*;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    // Account operations
    pub async fn create_account(&self, request: CreateAccountRequest) -> Result<Account> {
        let row = sqlx::query(
            r#"
            INSERT INTO accounts (code, name, account_type, parent_id)
            VALUES (?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&request.code)
        .bind(&request.name)
        .bind(&request.account_type)
        .bind(request.parent_id)
        .fetch_one(&self.pool)
        .await?;

        let account = Account {
            id: row.get("id"),
            code: row.get("code"),
            name: row.get("name"),
            account_type: row.get("account_type"),
            parent_id: row.get("parent_id"),
            balance: Decimal::try_from(row.get::<f64, _>("balance")).unwrap_or(Decimal::ZERO),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(account)
    }

    pub async fn get_account(&self, id: i64) -> Result<Option<Account>> {
        let row = sqlx::query("SELECT * FROM accounts WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let account = Account {
                id: row.get("id"),
                code: row.get("code"),
                name: row.get("name"),
                account_type: row.get("account_type"),
                parent_id: row.get("parent_id"),
                balance: Decimal::try_from(row.get::<f64, _>("balance")).unwrap_or(Decimal::ZERO),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Some(account))
        } else {
            Ok(None)
        }
    }

    pub async fn get_account_by_code(&self, code: &str) -> Result<Option<Account>> {
        let row = sqlx::query("SELECT * FROM accounts WHERE code = ?")
            .bind(code)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let account = Account {
                id: row.get("id"),
                code: row.get("code"),
                name: row.get("name"),
                account_type: row.get("account_type"),
                parent_id: row.get("parent_id"),
                balance: Decimal::try_from(row.get::<f64, _>("balance")).unwrap_or(Decimal::ZERO),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Some(account))
        } else {
            Ok(None)
        }
    }

    pub async fn list_accounts(&self) -> Result<Vec<Account>> {
        let rows = sqlx::query("SELECT * FROM accounts ORDER BY code")
            .fetch_all(&self.pool)
            .await?;

        let mut accounts = Vec::new();
        for row in rows {
            let account = Account {
                id: row.get("id"),
                code: row.get("code"),
                name: row.get("name"),
                account_type: row.get("account_type"),
                parent_id: row.get("parent_id"),
                balance: Decimal::try_from(row.get::<f64, _>("balance")).unwrap_or(Decimal::ZERO),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            accounts.push(account);
        }

        Ok(accounts)
    }

    pub async fn update_account(&self, id: i64, request: UpdateAccountRequest) -> Result<Option<Account>> {
        let mut query = "UPDATE accounts SET updated_at = CURRENT_TIMESTAMP".to_string();
        let mut bind_values = Vec::new();

        if let Some(name) = &request.name {
            query.push_str(", name = ?");
            bind_values.push(name.clone());
        }

        if let Some(parent_id) = request.parent_id {
            query.push_str(", parent_id = ?");
            bind_values.push(parent_id.to_string());
        }

        query.push_str(" WHERE id = ? RETURNING *");

        let mut sql_query = sqlx::query(&query);
        
        for value in &bind_values {
            sql_query = sql_query.bind(value);
        }
        
        sql_query = sql_query.bind(id);

        let row = sql_query.fetch_optional(&self.pool).await?;
        
        if let Some(row) = row {
            let account = Account {
                id: row.get("id"),
                code: row.get("code"),
                name: row.get("name"),
                account_type: row.get("account_type"),
                parent_id: row.get("parent_id"),
                balance: Decimal::try_from(row.get::<f64, _>("balance")).unwrap_or(Decimal::ZERO),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Some(account))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_account(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM accounts WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // Transaction operations
    pub async fn create_transaction(&self, request: CreateTransactionRequest) -> Result<TransactionWithEntries> {
        // Validate the transaction first
        request.validate()?;

        let mut tx = self.pool.begin().await?;

        // Create the transaction
        let row = sqlx::query(
            r#"
            INSERT INTO transactions (description, reference, transaction_date)
            VALUES (?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&request.description)
        .bind(&request.reference)
        .bind(request.transaction_date)
        .fetch_one(&mut *tx)
        .await?;

        let transaction = Transaction {
            id: row.get("id"),
            description: row.get("description"),
            reference: row.get("reference"),
            transaction_date: row.get("transaction_date"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        // Create journal entries
        let mut journal_entries = Vec::new();
        let mut total_debits = Decimal::ZERO;
        let mut total_credits = Decimal::ZERO;

        for entry_request in &request.journal_entries {
            let debit_amount = entry_request.debit_amount.unwrap_or(Decimal::ZERO);
            let credit_amount = entry_request.credit_amount.unwrap_or(Decimal::ZERO);

            total_debits += debit_amount;
            total_credits += credit_amount;

            // Insert journal entry
            sqlx::query(
                "INSERT INTO journal_entries (transaction_id, account_id, debit_amount, credit_amount, description) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(transaction.id)
            .bind(entry_request.account_id)
            .bind(f64::try_from(debit_amount).unwrap_or(0.0))
            .bind(f64::try_from(credit_amount).unwrap_or(0.0))
            .bind(&entry_request.description)
            .execute(&mut *tx)
            .await?;

            // Get the created entry with account details
            let entry_row = sqlx::query(
                r#"
                SELECT
                    je.id,
                    je.transaction_id,
                    je.account_id,
                    a.code as account_code,
                    a.name as account_name,
                    je.debit_amount,
                    je.credit_amount,
                    je.description,
                    je.created_at
                FROM journal_entries je
                JOIN accounts a ON je.account_id = a.id
                WHERE je.id = last_insert_rowid()
                "#,
            )
            .fetch_one(&mut *tx)
            .await?;

            let entry = JournalEntryWithAccount {
                id: entry_row.get("id"),
                transaction_id: entry_row.get("transaction_id"),
                account_id: entry_row.get("account_id"),
                account_code: entry_row.get("account_code"),
                account_name: entry_row.get("account_name"),
                debit_amount: Decimal::try_from(entry_row.get::<f64, _>("debit_amount")).unwrap_or(Decimal::ZERO),
                credit_amount: Decimal::try_from(entry_row.get::<f64, _>("credit_amount")).unwrap_or(Decimal::ZERO),
                description: entry_row.get("description"),
                created_at: entry_row.get("created_at"),
            };

            journal_entries.push(entry);
        }

        tx.commit().await?;

        Ok(TransactionWithEntries {
            transaction,
            journal_entries,
            total_debits,
            total_credits,
        })
    }

    pub async fn get_transaction(&self, id: i64) -> Result<Option<TransactionWithEntries>> {
        let transaction_row = sqlx::query("SELECT * FROM transactions WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(transaction_row) = transaction_row {
            let transaction = Transaction {
                id: transaction_row.get("id"),
                description: transaction_row.get("description"),
                reference: transaction_row.get("reference"),
                transaction_date: transaction_row.get("transaction_date"),
                created_at: transaction_row.get("created_at"),
                updated_at: transaction_row.get("updated_at"),
            };

            let entry_rows = sqlx::query(
                r#"
                SELECT
                    je.id,
                    je.transaction_id,
                    je.account_id,
                    a.code as account_code,
                    a.name as account_name,
                    je.debit_amount,
                    je.credit_amount,
                    je.description,
                    je.created_at
                FROM journal_entries je
                JOIN accounts a ON je.account_id = a.id
                WHERE je.transaction_id = ?
                ORDER BY je.id
                "#,
            )
            .bind(transaction.id)
            .fetch_all(&self.pool)
            .await?;

            let mut journal_entries = Vec::new();
            let mut total_debits = Decimal::ZERO;
            let mut total_credits = Decimal::ZERO;

            for row in entry_rows {
                let debit_amount = Decimal::try_from(row.get::<f64, _>("debit_amount")).unwrap_or(Decimal::ZERO);
                let credit_amount = Decimal::try_from(row.get::<f64, _>("credit_amount")).unwrap_or(Decimal::ZERO);
                
                total_debits += debit_amount;
                total_credits += credit_amount;

                let entry = JournalEntryWithAccount {
                    id: row.get("id"),
                    transaction_id: row.get("transaction_id"),
                    account_id: row.get("account_id"),
                    account_code: row.get("account_code"),
                    account_name: row.get("account_name"),
                    debit_amount,
                    credit_amount,
                    description: row.get("description"),
                    created_at: row.get("created_at"),
                };
                journal_entries.push(entry);
            }

            Ok(Some(TransactionWithEntries {
                transaction,
                journal_entries,
                total_debits,
                total_credits,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_transactions(&self, filter: TransactionFilter) -> Result<Vec<TransactionWithEntries>> {
        // For simplicity, let's use a basic query and filter in memory for now
        // In a production system, you'd want to optimize this with proper SQL filtering
        let transaction_rows = sqlx::query(
            "SELECT * FROM transactions ORDER BY transaction_date DESC, id DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut transactions = Vec::new();
        for row in transaction_rows {
            let transaction = Transaction {
                id: row.get("id"),
                description: row.get("description"),
                reference: row.get("reference"),
                transaction_date: row.get("transaction_date"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            transactions.push(transaction);
        }

        let mut result = Vec::new();
        for transaction in transactions {
            // Apply filters
            if let Some(start_date) = filter.start_date {
                if transaction.transaction_date < start_date {
                    continue;
                }
            }
            
            if let Some(end_date) = filter.end_date {
                if transaction.transaction_date > end_date {
                    continue;
                }
            }
            
            if let Some(ref description_filter) = filter.description_contains {
                if !transaction.description.to_lowercase().contains(&description_filter.to_lowercase()) {
                    continue;
                }
            }

            if let Some(transaction_with_entries) = self.get_transaction(transaction.id).await? {
                result.push(transaction_with_entries);
            }
            
            // Apply limit
            if let Some(limit) = filter.limit {
                if result.len() >= limit as usize {
                    break;
                }
            }
        }

        Ok(result)
    }

    pub async fn get_account_summary(&self) -> Result<AccountSummary> {
        let accounts = self.list_accounts().await?;
        let mut summary = AccountSummary::new();

        for account in accounts {
            summary.add_account(&account);
        }

        Ok(summary)
    }

    pub async fn get_trial_balance(&self) -> Result<TrialBalance> {
        let rows = sqlx::query(
            r#"
            SELECT
                a.id,
                a.code,
                a.name,
                a.account_type,
                COALESCE(SUM(je.debit_amount), 0.0) as total_debits,
                COALESCE(SUM(je.credit_amount), 0.0) as total_credits
            FROM accounts a
            LEFT JOIN journal_entries je ON a.id = je.account_id
            GROUP BY a.id, a.code, a.name, a.account_type
            ORDER BY a.code
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut trial_balance = TrialBalance::new();

        for row in rows {
            let account_type: AccountType = row.get("account_type");
            let total_debits = Decimal::try_from(row.get::<f64, _>("total_debits")).unwrap_or(Decimal::ZERO);
            let total_credits = Decimal::try_from(row.get::<f64, _>("total_credits")).unwrap_or(Decimal::ZERO);
            
            let net_balance = total_debits - total_credits;
            let (debit_balance, credit_balance) = if net_balance >= Decimal::ZERO {
                (net_balance, Decimal::ZERO)
            } else {
                (Decimal::ZERO, -net_balance)
            };

            let entry = TrialBalanceEntry {
                account_id: row.get("id"),
                account_code: row.get("code"),
                account_name: row.get("name"),
                account_type,
                debit_balance,
                credit_balance,
            };

            trial_balance.add_entry(entry);
        }

        trial_balance.sort_by_type_and_code();
        Ok(trial_balance)
    }
}