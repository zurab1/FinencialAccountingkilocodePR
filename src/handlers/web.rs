use axum::{
    extract::Extension,
    response::Html,
};
use std::sync::Arc;

use crate::{
    handlers::ApiError,
    AppState,
};

pub async fn dashboard(
    Extension(state): Extension<AppState>,
) -> Result<Html<String>, ApiError> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Financial Accounting System - Dashboard</title>
    <link rel="stylesheet" href="/static/css/styles.css">
</head>
<body>
    <nav class="navbar">
        <div class="nav-container">
            <h1 class="nav-title">Financial Accounting System</h1>
            <ul class="nav-menu">
                <li><a href="/" class="nav-link active">Dashboard</a></li>
                <li><a href="/accounts" class="nav-link">Accounts</a></li>
                <li><a href="/transactions" class="nav-link">Transactions</a></li>
                <li><a href="/reports" class="nav-link">Reports</a></li>
            </ul>
        </div>
    </nav>

    <main class="main-content">
        <div class="container">
            <h2>Dashboard</h2>
            
            <div class="dashboard-grid">
                <div class="card">
                    <h3>Account Summary</h3>
                    <div id="account-summary">
                        <div class="loading">Loading...</div>
                    </div>
                </div>
                
                <div class="card">
                    <h3>Recent Transactions</h3>
                    <div id="recent-transactions">
                        <div class="loading">Loading...</div>
                    </div>
                </div>
                
                <div class="card">
                    <h3>Quick Actions</h3>
                    <div class="quick-actions">
                        <button class="btn btn-primary" onclick="location.href='/transactions#new'">New Transaction</button>
                        <button class="btn btn-secondary" onclick="location.href='/accounts#new'">New Account</button>
                        <button class="btn btn-secondary" onclick="location.href='/reports/trial-balance'">Trial Balance</button>
                    </div>
                </div>
                
                <div class="card">
                    <h3>System Status</h3>
                    <div class="status-indicators">
                        <div class="status-item">
                            <span class="status-label">Database:</span>
                            <span class="status-value status-ok">Connected</span>
                        </div>
                        <div class="status-item">
                            <span class="status-label">Balance Check:</span>
                            <span class="status-value" id="balance-status">Checking...</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </main>

    <script src="/static/js/dashboard.js"></script>
</body>
</html>
    "#;

    Ok(Html(html.to_string()))
}

pub async fn accounts_page(
    Extension(state): Extension<AppState>,
) -> Result<Html<String>, ApiError> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Accounts - Financial Accounting System</title>
    <link rel="stylesheet" href="/static/css/styles.css">
</head>
<body>
    <nav class="navbar">
        <div class="nav-container">
            <h1 class="nav-title">Financial Accounting System</h1>
            <ul class="nav-menu">
                <li><a href="/" class="nav-link">Dashboard</a></li>
                <li><a href="/accounts" class="nav-link active">Accounts</a></li>
                <li><a href="/transactions" class="nav-link">Transactions</a></li>
                <li><a href="/reports" class="nav-link">Reports</a></li>
            </ul>
        </div>
    </nav>

    <main class="main-content">
        <div class="container">
            <div class="page-header">
                <h2>Chart of Accounts</h2>
                <button class="btn btn-primary" onclick="showNewAccountForm()">New Account</button>
            </div>
            
            <div class="filters">
                <select id="account-type-filter" onchange="filterAccounts()">
                    <option value="">All Account Types</option>
                    <option value="asset">Assets</option>
                    <option value="liability">Liabilities</option>
                    <option value="equity">Equity</option>
                    <option value="revenue">Revenue</option>
                    <option value="expense">Expenses</option>
                </select>
            </div>

            <div class="card">
                <div id="accounts-table">
                    <div class="loading">Loading accounts...</div>
                </div>
            </div>
        </div>
    </main>

    <!-- New Account Modal -->
    <div id="new-account-modal" class="modal">
        <div class="modal-content">
            <div class="modal-header">
                <h3>New Account</h3>
                <span class="close" onclick="hideNewAccountForm()">&times;</span>
            </div>
            <form id="new-account-form">
                <div class="form-group">
                    <label for="account-code">Account Code:</label>
                    <input type="text" id="account-code" name="code" required>
                </div>
                <div class="form-group">
                    <label for="account-name">Account Name:</label>
                    <input type="text" id="account-name" name="name" required>
                </div>
                <div class="form-group">
                    <label for="account-type">Account Type:</label>
                    <select id="account-type" name="account_type" required>
                        <option value="">Select Type</option>
                        <option value="asset">Asset</option>
                        <option value="liability">Liability</option>
                        <option value="equity">Equity</option>
                        <option value="revenue">Revenue</option>
                        <option value="expense">Expense</option>
                    </select>
                </div>
                <div class="form-actions">
                    <button type="button" class="btn btn-secondary" onclick="hideNewAccountForm()">Cancel</button>
                    <button type="submit" class="btn btn-primary">Create Account</button>
                </div>
            </form>
        </div>
    </div>

    <script src="/static/js/accounts.js"></script>
</body>
</html>
    "#;

    Ok(Html(html.to_string()))
}

pub async fn transactions_page(
    Extension(state): Extension<AppState>,
) -> Result<Html<String>, ApiError> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Transactions - Financial Accounting System</title>
    <link rel="stylesheet" href="/static/css/styles.css">
</head>
<body>
    <nav class="navbar">
        <div class="nav-container">
            <h1 class="nav-title">Financial Accounting System</h1>
            <ul class="nav-menu">
                <li><a href="/" class="nav-link">Dashboard</a></li>
                <li><a href="/accounts" class="nav-link">Accounts</a></li>
                <li><a href="/transactions" class="nav-link active">Transactions</a></li>
                <li><a href="/reports" class="nav-link">Reports</a></li>
            </ul>
        </div>
    </nav>

    <main class="main-content">
        <div class="container">
            <div class="page-header">
                <h2>Transactions</h2>
                <button class="btn btn-primary" onclick="showNewTransactionForm()">New Transaction</button>
            </div>
            
            <div class="filters">
                <input type="date" id="start-date" onchange="filterTransactions()">
                <input type="date" id="end-date" onchange="filterTransactions()">
                <input type="text" id="description-filter" placeholder="Search description..." onchange="filterTransactions()">
            </div>

            <div class="card">
                <div id="transactions-table">
                    <div class="loading">Loading transactions...</div>
                </div>
            </div>
        </div>
    </main>

    <!-- New Transaction Modal -->
    <div id="new-transaction-modal" class="modal">
        <div class="modal-content large">
            <div class="modal-header">
                <h3>New Transaction</h3>
                <span class="close" onclick="hideNewTransactionForm()">&times;</span>
            </div>
            <form id="new-transaction-form">
                <div class="form-group">
                    <label for="transaction-description">Description:</label>
                    <input type="text" id="transaction-description" name="description" required>
                </div>
                <div class="form-group">
                    <label for="transaction-date">Date:</label>
                    <input type="date" id="transaction-date" name="transaction_date" required>
                </div>
                <div class="form-group">
                    <label for="transaction-reference">Reference (optional):</label>
                    <input type="text" id="transaction-reference" name="reference">
                </div>
                
                <h4>Journal Entries</h4>
                <div id="journal-entries">
                    <!-- Journal entries will be added dynamically -->
                </div>
                
                <div class="journal-entry-actions">
                    <button type="button" class="btn btn-secondary" onclick="addJournalEntry()">Add Entry</button>
                    <div class="balance-check">
                        <span>Total Debits: $<span id="total-debits">0.00</span></span>
                        <span>Total Credits: $<span id="total-credits">0.00</span></span>
                        <span id="balance-status" class="balance-status"></span>
                    </div>
                </div>
                
                <div class="form-actions">
                    <button type="button" class="btn btn-secondary" onclick="hideNewTransactionForm()">Cancel</button>
                    <button type="submit" class="btn btn-primary" id="submit-transaction" disabled>Create Transaction</button>
                </div>
            </form>
        </div>
    </div>

    <script src="/static/js/transactions.js"></script>
</body>
</html>
    "#;

    Ok(Html(html.to_string()))
}

pub async fn reports_page(
    Extension(state): Extension<AppState>,
) -> Result<Html<String>, ApiError> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Reports - Financial Accounting System</title>
    <link rel="stylesheet" href="/static/css/styles.css">
</head>
<body>
    <nav class="navbar">
        <div class="nav-container">
            <h1 class="nav-title">Financial Accounting System</h1>
            <ul class="nav-menu">
                <li><a href="/" class="nav-link">Dashboard</a></li>
                <li><a href="/accounts" class="nav-link">Accounts</a></li>
                <li><a href="/transactions" class="nav-link">Transactions</a></li>
                <li><a href="/reports" class="nav-link active">Reports</a></li>
            </ul>
        </div>
    </nav>

    <main class="main-content">
        <div class="container">
            <h2>Financial Reports</h2>
            
            <div class="reports-grid">
                <div class="card report-card" onclick="location.href='/reports/trial-balance'">
                    <h3>Trial Balance</h3>
                    <p>View all account balances to ensure debits equal credits</p>
                </div>
                
                <div class="card report-card" onclick="loadBalanceSheet()">
                    <h3>Balance Sheet</h3>
                    <p>Assets, Liabilities, and Equity statement</p>
                </div>
                
                <div class="card report-card" onclick="loadIncomeStatement()">
                    <h3>Income Statement</h3>
                    <p>Revenue and Expenses for the period</p>
                </div>
                
                <div class="card report-card" onclick="loadAccountSummary()">
                    <h3>Account Summary</h3>
                    <p>Overview of all account types and totals</p>
                </div>
            </div>
            
            <div id="report-content" class="card" style="display: none;">
                <div class="report-header">
                    <h3 id="report-title"></h3>
                    <button class="btn btn-secondary" onclick="printReport()">Print</button>
                </div>
                <div id="report-data">
                    <!-- Report data will be loaded here -->
                </div>
            </div>
        </div>
    </main>

    <script src="/static/js/reports.js"></script>
</body>
</html>
    "#;

    Ok(Html(html.to_string()))
}

pub async fn trial_balance_page(
    Extension(state): Extension<AppState>,
) -> Result<Html<String>, ApiError> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Trial Balance - Financial Accounting System</title>
    <link rel="stylesheet" href="/static/css/styles.css">
</head>
<body>
    <nav class="navbar">
        <div class="nav-container">
            <h1 class="nav-title">Financial Accounting System</h1>
            <ul class="nav-menu">
                <li><a href="/" class="nav-link">Dashboard</a></li>
                <li><a href="/accounts" class="nav-link">Accounts</a></li>
                <li><a href="/transactions" class="nav-link">Transactions</a></li>
                <li><a href="/reports" class="nav-link active">Reports</a></li>
            </ul>
        </div>
    </nav>

    <main class="main-content">
        <div class="container">
            <div class="page-header">
                <h2>Trial Balance</h2>
                <div class="report-actions">
                    <button class="btn btn-secondary" onclick="window.print()">Print</button>
                    <button class="btn btn-secondary" onclick="location.href='/reports'">Back to Reports</button>
                </div>
            </div>
            
            <div class="card">
                <div id="trial-balance-content">
                    <div class="loading">Loading trial balance...</div>
                </div>
            </div>
        </div>
    </main>

    <script src="/static/js/trial-balance.js"></script>
</body>
</html>
    "#;

    Ok(Html(html.to_string()))
}