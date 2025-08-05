use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct JournalEntry {
    pub id: i64,
    pub transaction_id: i64,
    pub account_id: i64,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJournalEntryRequest {
    pub transaction_id: i64,
    pub account_id: i64,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateJournalEntryRequest {
    pub account_id: Option<i64>,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct JournalEntryWithDetails {
    pub id: i64,
    pub transaction_id: i64,
    pub transaction_description: String,
    pub transaction_date: chrono::NaiveDate,
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl JournalEntry {
    /// Get the net effect of this journal entry (positive for debits, negative for credits)
    pub fn net_amount(&self) -> Decimal {
        self.debit_amount - self.credit_amount
    }

    /// Check if this is a debit entry
    pub fn is_debit(&self) -> bool {
        self.debit_amount > Decimal::ZERO
    }

    /// Check if this is a credit entry
    pub fn is_credit(&self) -> bool {
        self.credit_amount > Decimal::ZERO
    }

    /// Get the absolute amount of this entry
    pub fn amount(&self) -> Decimal {
        if self.is_debit() {
            self.debit_amount
        } else {
            self.credit_amount
        }
    }
}

impl CreateJournalEntryRequest {
    /// Validate the journal entry request
    pub fn validate(&self) -> Result<(), String> {
        match (self.debit_amount, self.credit_amount) {
            (Some(debit), None) => {
                if debit <= Decimal::ZERO {
                    return Err("Debit amount must be positive".to_string());
                }
            }
            (None, Some(credit)) => {
                if credit <= Decimal::ZERO {
                    return Err("Credit amount must be positive".to_string());
                }
            }
            (Some(_), Some(_)) => {
                return Err("Journal entry cannot have both debit and credit amounts".to_string());
            }
            (None, None) => {
                return Err("Journal entry must have either debit or credit amount".to_string());
            }
        }
        Ok(())
    }

    /// Convert to a JournalEntry (without ID and timestamps)
    pub fn to_journal_entry(&self) -> JournalEntry {
        JournalEntry {
            id: 0, // Will be set by database
            transaction_id: self.transaction_id,
            account_id: self.account_id,
            debit_amount: self.debit_amount.unwrap_or(Decimal::ZERO),
            credit_amount: self.credit_amount.unwrap_or(Decimal::ZERO),
            description: self.description.clone(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrialBalance {
    pub entries: Vec<TrialBalanceEntry>,
    pub total_debits: Decimal,
    pub total_credits: Decimal,
    pub is_balanced: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrialBalanceEntry {
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub account_type: crate::models::AccountType,
    pub debit_balance: Decimal,
    pub credit_balance: Decimal,
}

impl TrialBalance {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            total_debits: Decimal::ZERO,
            total_credits: Decimal::ZERO,
            is_balanced: true,
        }
    }

    pub fn add_entry(&mut self, entry: TrialBalanceEntry) {
        self.total_debits += entry.debit_balance;
        self.total_credits += entry.credit_balance;
        self.entries.push(entry);
        self.is_balanced = self.total_debits == self.total_credits;
    }

    /// Sort entries by account code
    pub fn sort_by_code(&mut self) {
        self.entries.sort_by(|a, b| a.account_code.cmp(&b.account_code));
    }

    /// Sort entries by account type and then by code
    pub fn sort_by_type_and_code(&mut self) {
        self.entries.sort_by(|a, b| {
            let type_order = |t: &crate::models::AccountType| match t {
                crate::models::AccountType::Asset => 1,
                crate::models::AccountType::Liability => 2,
                crate::models::AccountType::Equity => 3,
                crate::models::AccountType::Revenue => 4,
                crate::models::AccountType::Expense => 5,
            };
            
            let a_order = type_order(&a.account_type);
            let b_order = type_order(&b.account_type);
            
            a_order.cmp(&b_order).then_with(|| a.account_code.cmp(&b.account_code))
        });
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountStatement {
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub account_type: crate::models::AccountType,
    pub opening_balance: Decimal,
    pub closing_balance: Decimal,
    pub entries: Vec<JournalEntryWithDetails>,
    pub total_debits: Decimal,
    pub total_credits: Decimal,
}

impl AccountStatement {
    pub fn new(
        account_id: i64,
        account_code: String,
        account_name: String,
        account_type: crate::models::AccountType,
        opening_balance: Decimal,
    ) -> Self {
        Self {
            account_id,
            account_code,
            account_name,
            account_type,
            opening_balance,
            closing_balance: opening_balance,
            entries: Vec::new(),
            total_debits: Decimal::ZERO,
            total_credits: Decimal::ZERO,
        }
    }

    pub fn add_entry(&mut self, entry: JournalEntryWithDetails) {
        self.total_debits += entry.debit_amount;
        self.total_credits += entry.credit_amount;
        self.closing_balance += entry.debit_amount - entry.credit_amount;
        self.entries.push(entry);
    }

    /// Sort entries by transaction date
    pub fn sort_by_date(&mut self) {
        self.entries.sort_by(|a, b| a.transaction_date.cmp(&b.transaction_date));
    }
}