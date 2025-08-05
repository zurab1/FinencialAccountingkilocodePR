use axum::{
    extract::Extension,
    response::Json,
};
use std::sync::Arc;

use crate::{
    models::*,
    database::Database,
    handlers::ApiError,
    AppState,
};

pub async fn account_summary(
    Extension(state): Extension<AppState>,
) -> Result<Json<AccountSummary>, ApiError> {
    let summary = state.database.get_account_summary().await?;
    Ok(Json(summary))
}

pub async fn trial_balance(
    Extension(state): Extension<AppState>,
) -> Result<Json<TrialBalance>, ApiError> {
    let trial_balance = state.database.get_trial_balance().await?;
    Ok(Json(trial_balance))
}

pub async fn balance_sheet(
    Extension(state): Extension<AppState>,
) -> Result<Json<BalanceSheet>, ApiError> {
    let accounts = state.database.list_accounts().await?;
    let mut balance_sheet = BalanceSheet::new();

    for account in accounts {
        balance_sheet.add_account(&account);
    }

    Ok(Json(balance_sheet))
}

pub async fn income_statement(
    Extension(state): Extension<AppState>,
) -> Result<Json<IncomeStatement>, ApiError> {
    let accounts = state.database.list_accounts().await?;
    let mut income_statement = IncomeStatement::new();

    for account in accounts {
        if matches!(account.account_type, AccountType::Revenue | AccountType::Expense) {
            income_statement.add_account(&account);
        }
    }

    Ok(Json(income_statement))
}

// Additional report structures
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BalanceSheet {
    pub assets: BalanceSheetSection,
    pub liabilities: BalanceSheetSection,
    pub equity: BalanceSheetSection,
    pub total_assets: rust_decimal::Decimal,
    pub total_liabilities_and_equity: rust_decimal::Decimal,
    pub is_balanced: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BalanceSheetSection {
    pub accounts: Vec<BalanceSheetAccount>,
    pub total: rust_decimal::Decimal,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BalanceSheetAccount {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub balance: rust_decimal::Decimal,
}

impl BalanceSheet {
    pub fn new() -> Self {
        Self {
            assets: BalanceSheetSection {
                accounts: Vec::new(),
                total: rust_decimal::Decimal::ZERO,
            },
            liabilities: BalanceSheetSection {
                accounts: Vec::new(),
                total: rust_decimal::Decimal::ZERO,
            },
            equity: BalanceSheetSection {
                accounts: Vec::new(),
                total: rust_decimal::Decimal::ZERO,
            },
            total_assets: rust_decimal::Decimal::ZERO,
            total_liabilities_and_equity: rust_decimal::Decimal::ZERO,
            is_balanced: true,
        }
    }

    pub fn add_account(&mut self, account: &Account) {
        let balance_sheet_account = BalanceSheetAccount {
            id: account.id,
            code: account.code.clone(),
            name: account.name.clone(),
            balance: account.normal_balance().abs(),
        };

        match account.account_type {
            AccountType::Asset => {
                self.assets.total += balance_sheet_account.balance;
                self.assets.accounts.push(balance_sheet_account);
            }
            AccountType::Liability => {
                self.liabilities.total += balance_sheet_account.balance;
                self.liabilities.accounts.push(balance_sheet_account);
            }
            AccountType::Equity => {
                self.equity.total += balance_sheet_account.balance;
                self.equity.accounts.push(balance_sheet_account);
            }
            _ => {} // Revenue and Expense accounts don't appear on balance sheet
        }

        self.total_assets = self.assets.total;
        self.total_liabilities_and_equity = self.liabilities.total + self.equity.total;
        self.is_balanced = self.total_assets == self.total_liabilities_and_equity;
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IncomeStatement {
    pub revenue: IncomeStatementSection,
    pub expenses: IncomeStatementSection,
    pub gross_profit: rust_decimal::Decimal,
    pub net_income: rust_decimal::Decimal,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IncomeStatementSection {
    pub accounts: Vec<IncomeStatementAccount>,
    pub total: rust_decimal::Decimal,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IncomeStatementAccount {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub amount: rust_decimal::Decimal,
}

impl IncomeStatement {
    pub fn new() -> Self {
        Self {
            revenue: IncomeStatementSection {
                accounts: Vec::new(),
                total: rust_decimal::Decimal::ZERO,
            },
            expenses: IncomeStatementSection {
                accounts: Vec::new(),
                total: rust_decimal::Decimal::ZERO,
            },
            gross_profit: rust_decimal::Decimal::ZERO,
            net_income: rust_decimal::Decimal::ZERO,
        }
    }

    pub fn add_account(&mut self, account: &Account) {
        let income_account = IncomeStatementAccount {
            id: account.id,
            code: account.code.clone(),
            name: account.name.clone(),
            amount: account.normal_balance().abs(),
        };

        match account.account_type {
            AccountType::Revenue => {
                self.revenue.total += income_account.amount;
                self.revenue.accounts.push(income_account);
            }
            AccountType::Expense => {
                self.expenses.total += income_account.amount;
                self.expenses.accounts.push(income_account);
            }
            _ => {} // Only revenue and expense accounts appear on income statement
        }

        self.gross_profit = self.revenue.total;
        self.net_income = self.revenue.total - self.expenses.total;
    }
}