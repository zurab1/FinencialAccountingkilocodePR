use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub description: String,
    pub reference: Option<String>,
    pub transaction_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub description: String,
    pub reference: Option<String>,
    pub transaction_date: NaiveDate,
    pub journal_entries: Vec<CreateJournalEntryRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJournalEntryRequest {
    pub account_id: i64,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTransactionRequest {
    pub description: Option<String>,
    pub reference: Option<String>,
    pub transaction_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionWithEntries {
    pub transaction: Transaction,
    pub journal_entries: Vec<JournalEntryWithAccount>,
    pub total_debits: Decimal,
    pub total_credits: Decimal,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct JournalEntryWithAccount {
    pub id: i64,
    pub transaction_id: i64,
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl CreateTransactionRequest {
    /// Validate that the transaction balances (total debits = total credits)
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.journal_entries.is_empty() {
            return Err(anyhow::anyhow!("Transaction must have at least one journal entry"));
        }

        let mut total_debits = Decimal::ZERO;
        let mut total_credits = Decimal::ZERO;

        for entry in &self.journal_entries {
            match (entry.debit_amount, entry.credit_amount) {
                (Some(debit), None) => {
                    if debit <= Decimal::ZERO {
                        return Err(anyhow::anyhow!("Debit amounts must be positive"));
                    }
                    total_debits += debit;
                }
                (None, Some(credit)) => {
                    if credit <= Decimal::ZERO {
                        return Err(anyhow::anyhow!("Credit amounts must be positive"));
                    }
                    total_credits += credit;
                }
                (Some(_), Some(_)) => {
                    return Err(anyhow::anyhow!("Journal entry cannot have both debit and credit amounts"));
                }
                (None, None) => {
                    return Err(anyhow::anyhow!("Journal entry must have either debit or credit amount"));
                }
            }
        }

        if total_debits != total_credits {
            return Err(anyhow::anyhow!(
                "Transaction does not balance: debits ({}) != credits ({})",
                total_debits, total_credits
            ));
        }

        Ok(())
    }

    /// Get the total amount of the transaction (sum of debits or credits)
    pub fn total_amount(&self) -> Decimal {
        self.journal_entries
            .iter()
            .map(|entry| {
                entry.debit_amount.unwrap_or_else(|| entry.credit_amount.unwrap_or(Decimal::ZERO))
            })
            .fold(Decimal::ZERO, |acc, amount| acc + amount) / Decimal::from(2)
    }
}

impl TransactionWithEntries {
    /// Check if the transaction is balanced
    pub fn is_balanced(&self) -> bool {
        self.total_debits == self.total_credits
    }

    /// Get the net amount of the transaction
    pub fn net_amount(&self) -> Decimal {
        self.total_debits.max(self.total_credits)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub total_transactions: i64,
    pub total_amount: Decimal,
    pub date_range: Option<(NaiveDate, NaiveDate)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionFilter {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub account_id: Option<i64>,
    pub description_contains: Option<String>,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Default for TransactionFilter {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            account_id: None,
            description_contains: None,
            min_amount: None,
            max_amount: None,
            limit: Some(50),
            offset: Some(0),
        }
    }
}
impl CreateJournalEntryRequest {
    pub fn validate(&self) -> anyhow::Result<()> {
        let debit_zero = self.debit_amount.map_or(true, |d| d.is_zero());
        let credit_zero = self.credit_amount.map_or(true, |c| c.is_zero());
        
        if debit_zero && credit_zero {
            return Err(anyhow::anyhow!("Either debit or credit amount must be non-zero"));
        }
        if !debit_zero && !credit_zero {
            return Err(anyhow::anyhow!("Cannot have both debit and credit amounts"));
        }
        Ok(())
    }
}
