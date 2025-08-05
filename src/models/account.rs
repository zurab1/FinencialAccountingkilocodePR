use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<i64>,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl AccountType {
    /// Returns true if this account type normally has a debit balance
    pub fn is_debit_normal(&self) -> bool {
        matches!(self, AccountType::Asset | AccountType::Expense)
    }

    /// Returns true if this account type normally has a credit balance
    pub fn is_credit_normal(&self) -> bool {
        matches!(self, AccountType::Liability | AccountType::Equity | AccountType::Revenue)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub account_type: AccountType,
    pub balance: Decimal,
    pub debit_total: Decimal,
    pub credit_total: Decimal,
}

impl Account {
    /// Calculate the normal balance for this account type
    pub fn normal_balance(&self) -> Decimal {
        if self.account_type.is_debit_normal() {
            self.balance
        } else {
            -self.balance
        }
    }

    /// Check if the account has a normal balance (positive for debit accounts, negative for credit accounts)
    pub fn has_normal_balance(&self) -> bool {
        if self.account_type.is_debit_normal() {
            self.balance >= Decimal::ZERO
        } else {
            self.balance <= Decimal::ZERO
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSummary {
    pub total_assets: Decimal,
    pub total_liabilities: Decimal,
    pub total_equity: Decimal,
    pub total_revenue: Decimal,
    pub total_expenses: Decimal,
    pub net_income: Decimal,
}

impl AccountSummary {
    pub fn new() -> Self {
        Self {
            total_assets: Decimal::ZERO,
            total_liabilities: Decimal::ZERO,
            total_equity: Decimal::ZERO,
            total_revenue: Decimal::ZERO,
            total_expenses: Decimal::ZERO,
            net_income: Decimal::ZERO,
        }
    }

    pub fn add_account(&mut self, account: &Account) {
        let balance = account.normal_balance().abs();
        
        match account.account_type {
            AccountType::Asset => self.total_assets += balance,
            AccountType::Liability => self.total_liabilities += balance,
            AccountType::Equity => self.total_equity += balance,
            AccountType::Revenue => self.total_revenue += balance,
            AccountType::Expense => self.total_expenses += balance,
        }
        
        self.net_income = self.total_revenue - self.total_expenses;
    }

    /// Check if the accounting equation balances: Assets = Liabilities + Equity + Net Income
    pub fn is_balanced(&self) -> bool {
        self.total_assets == self.total_liabilities + self.total_equity + self.net_income
    }
}