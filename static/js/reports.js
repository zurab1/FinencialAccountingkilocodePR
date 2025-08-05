// Reports page functionality
document.addEventListener('DOMContentLoaded', function() {
    // Reports page is ready
});

async function loadTrialBalance() {
    showReportContent('Trial Balance');
    
    try {
        const response = await fetch('/api/reports/trial-balance');
        const trialBalance = await response.json();
        
        const reportHtml = `
            <div class="trial-balance-report">
                <div class="balance-status-header">
                    <span class="balance-indicator ${trialBalance.is_balanced ? 'balanced' : 'unbalanced'}">
                        ${trialBalance.is_balanced ? 'Balanced ✓' : 'Unbalanced ⚠'}
                    </span>
                </div>
                
                <table class="table">
                    <thead>
                        <tr>
                            <th>Account</th>
                            <th>Type</th>
                            <th class="amount">Debit</th>
                            <th class="amount">Credit</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${trialBalance.entries.map(entry => `
                            <tr>
                                <td>
                                    <div class="account-info">
                                        <strong>${escapeHtml(entry.account_code)}</strong>
                                        <div class="account-name">${escapeHtml(entry.account_name)}</div>
                                    </div>
                                </td>
                                <td>
                                    <span class="account-type ${entry.account_type}">
                                        ${capitalizeFirst(entry.account_type)}
                                    </span>
                                </td>
                                <td class="amount debit">
                                    ${entry.debit_balance > 0 ? '$' + formatAmount(entry.debit_balance) : '-'}
                                </td>
                                <td class="amount credit">
                                    ${entry.credit_balance > 0 ? '$' + formatAmount(entry.credit_balance) : '-'}
                                </td>
                            </tr>
                        `).join('')}
                    </tbody>
                    <tfoot>
                        <tr class="totals">
                            <td colspan="2"><strong>TOTALS</strong></td>
                            <td class="amount debit"><strong>$${formatAmount(trialBalance.total_debits)}</strong></td>
                            <td class="amount credit"><strong>$${formatAmount(trialBalance.total_credits)}</strong></td>
                        </tr>
                    </tfoot>
                </table>
            </div>
        `;
        
        document.getElementById('report-data').innerHTML = reportHtml;
    } catch (error) {
        console.error('Error loading trial balance:', error);
        document.getElementById('report-data').innerHTML = 
            '<div class="error">Failed to load trial balance</div>';
    }
}

async function loadBalanceSheet() {
    showReportContent('Balance Sheet');
    
    try {
        const response = await fetch('/api/reports/balance-sheet');
        const balanceSheet = await response.json();
        
        const reportHtml = `
            <div class="balance-sheet-report">
                <div class="balance-sheet-sections">
                    <div class="balance-sheet-section">
                        <h4>Assets</h4>
                        <table class="table">
                            <tbody>
                                ${balanceSheet.assets.accounts.map(account => `
                                    <tr>
                                        <td>${escapeHtml(account.code)} - ${escapeHtml(account.name)}</td>
                                        <td class="amount">$${formatAmount(account.balance)}</td>
                                    </tr>
                                `).join('')}
                                <tr class="section-total">
                                    <td><strong>Total Assets</strong></td>
                                    <td class="amount"><strong>$${formatAmount(balanceSheet.assets.total)}</strong></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    
                    <div class="balance-sheet-section">
                        <h4>Liabilities</h4>
                        <table class="table">
                            <tbody>
                                ${balanceSheet.liabilities.accounts.map(account => `
                                    <tr>
                                        <td>${escapeHtml(account.code)} - ${escapeHtml(account.name)}</td>
                                        <td class="amount">$${formatAmount(account.balance)}</td>
                                    </tr>
                                `).join('')}
                                <tr class="section-total">
                                    <td><strong>Total Liabilities</strong></td>
                                    <td class="amount"><strong>$${formatAmount(balanceSheet.liabilities.total)}</strong></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    
                    <div class="balance-sheet-section">
                        <h4>Equity</h4>
                        <table class="table">
                            <tbody>
                                ${balanceSheet.equity.accounts.map(account => `
                                    <tr>
                                        <td>${escapeHtml(account.code)} - ${escapeHtml(account.name)}</td>
                                        <td class="amount">$${formatAmount(account.balance)}</td>
                                    </tr>
                                `).join('')}
                                <tr class="section-total">
                                    <td><strong>Total Equity</strong></td>
                                    <td class="amount"><strong>$${formatAmount(balanceSheet.equity.total)}</strong></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
                
                <div class="balance-verification">
                    <table class="table">
                        <tbody>
                            <tr class="verification-row">
                                <td><strong>Total Assets</strong></td>
                                <td class="amount"><strong>$${formatAmount(balanceSheet.total_assets)}</strong></td>
                            </tr>
                            <tr class="verification-row">
                                <td><strong>Total Liabilities + Equity</strong></td>
                                <td class="amount"><strong>$${formatAmount(balanceSheet.total_liabilities_and_equity)}</strong></td>
                            </tr>
                            <tr class="balance-check ${balanceSheet.is_balanced ? 'balanced' : 'unbalanced'}">
                                <td><strong>Balance Check</strong></td>
                                <td class="amount">
                                    <strong>${balanceSheet.is_balanced ? 'Balanced ✓' : 'Unbalanced ⚠'}</strong>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        `;
        
        document.getElementById('report-data').innerHTML = reportHtml;
    } catch (error) {
        console.error('Error loading balance sheet:', error);
        document.getElementById('report-data').innerHTML = 
            '<div class="error">Failed to load balance sheet</div>';
    }
}

async function loadIncomeStatement() {
    showReportContent('Income Statement');
    
    try {
        const response = await fetch('/api/reports/income-statement');
        const incomeStatement = await response.json();
        
        const reportHtml = `
            <div class="income-statement-report">
                <div class="income-statement-sections">
                    <div class="income-section">
                        <h4>Revenue</h4>
                        <table class="table">
                            <tbody>
                                ${incomeStatement.revenue.accounts.map(account => `
                                    <tr>
                                        <td>${escapeHtml(account.code)} - ${escapeHtml(account.name)}</td>
                                        <td class="amount revenue">$${formatAmount(account.amount)}</td>
                                    </tr>
                                `).join('')}
                                <tr class="section-total">
                                    <td><strong>Total Revenue</strong></td>
                                    <td class="amount revenue"><strong>$${formatAmount(incomeStatement.revenue.total)}</strong></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    
                    <div class="expense-section">
                        <h4>Expenses</h4>
                        <table class="table">
                            <tbody>
                                ${incomeStatement.expenses.accounts.map(account => `
                                    <tr>
                                        <td>${escapeHtml(account.code)} - ${escapeHtml(account.name)}</td>
                                        <td class="amount expense">$${formatAmount(account.amount)}</td>
                                    </tr>
                                `).join('')}
                                <tr class="section-total">
                                    <td><strong>Total Expenses</strong></td>
                                    <td class="amount expense"><strong>$${formatAmount(incomeStatement.expenses.total)}</strong></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
                
                <div class="net-income-section">
                    <table class="table">
                        <tbody>
                            <tr class="net-income ${incomeStatement.net_income >= 0 ? 'profit' : 'loss'}">
                                <td><strong>Net ${incomeStatement.net_income >= 0 ? 'Income' : 'Loss'}</strong></td>
                                <td class="amount">
                                    <strong>$${formatAmount(Math.abs(incomeStatement.net_income))}</strong>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        `;
        
        document.getElementById('report-data').innerHTML = reportHtml;
    } catch (error) {
        console.error('Error loading income statement:', error);
        document.getElementById('report-data').innerHTML = 
            '<div class="error">Failed to load income statement</div>';
    }
}

async function loadAccountSummary() {
    showReportContent('Account Summary');
    
    try {
        const response = await fetch('/api/reports/summary');
        const summary = await response.json();
        
        const reportHtml = `
            <div class="account-summary-report">
                <div class="summary-cards">
                    <div class="summary-card assets">
                        <h4>Total Assets</h4>
                        <div class="summary-amount">$${formatAmount(summary.total_assets)}</div>
                    </div>
                    
                    <div class="summary-card liabilities">
                        <h4>Total Liabilities</h4>
                        <div class="summary-amount">$${formatAmount(summary.total_liabilities)}</div>
                    </div>
                    
                    <div class="summary-card equity">
                        <h4>Total Equity</h4>
                        <div class="summary-amount">$${formatAmount(summary.total_equity)}</div>
                    </div>
                    
                    <div class="summary-card revenue">
                        <h4>Total Revenue</h4>
                        <div class="summary-amount">$${formatAmount(summary.total_revenue)}</div>
                    </div>
                    
                    <div class="summary-card expenses">
                        <h4>Total Expenses</h4>
                        <div class="summary-amount">$${formatAmount(summary.total_expenses)}</div>
                    </div>
                    
                    <div class="summary-card net-income ${summary.net_income >= 0 ? 'profit' : 'loss'}">
                        <h4>Net ${summary.net_income >= 0 ? 'Income' : 'Loss'}</h4>
                        <div class="summary-amount">$${formatAmount(Math.abs(summary.net_income))}</div>
                    </div>
                </div>
                
                <div class="accounting-equation">
                    <h4>Accounting Equation Verification</h4>
                    <div class="equation">
                        <div class="equation-part">
                            <span class="label">Assets</span>
                            <span class="amount">$${formatAmount(summary.total_assets)}</span>
                        </div>
                        <div class="equation-operator">=</div>
                        <div class="equation-part">
                            <span class="label">Liabilities</span>
                            <span class="amount">$${formatAmount(summary.total_liabilities)}</span>
                        </div>
                        <div class="equation-operator">+</div>
                        <div class="equation-part">
                            <span class="label">Equity</span>
                            <span class="amount">$${formatAmount(summary.total_equity)}</span>
                        </div>
                        <div class="equation-operator">+</div>
                        <div class="equation-part">
                            <span class="label">Net Income</span>
                            <span class="amount">$${formatAmount(summary.net_income)}</span>
                        </div>
                    </div>
                    <div class="equation-status ${summary.is_balanced ? 'balanced' : 'unbalanced'}">
                        ${summary.is_balanced ? 'Equation Balanced ✓' : 'Equation Unbalanced ⚠'}
                    </div>
                </div>
            </div>
        `;
        
        document.getElementById('report-data').innerHTML = reportHtml;
    } catch (error) {
        console.error('Error loading account summary:', error);
        document.getElementById('report-data').innerHTML = 
            '<div class="error">Failed to load account summary</div>';
    }
}

function showReportContent(title) {
    document.getElementById('report-title').textContent = title;
    document.getElementById('report-content').style.display = 'block';
    document.getElementById('report-data').innerHTML = '<div class="loading">Loading report...</div>';
}

function printReport() {
    window.print();
}

// Utility functions
function formatAmount(amount) {
    return parseFloat(amount).toLocaleString('en-US', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
    });
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function capitalizeFirst(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}

// Add reports-specific styles
const reportsStyles = `
<style>
.balance-status-header {
    text-align: center;
    margin-bottom: 1.5rem;
}

.balance-indicator {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-weight: 600;
    font-size: 1.1rem;
    display: inline-block;
}

.balance-indicator.balanced {
    background-color: #d4edda;
    color: #155724;
    border: 2px solid #c3e6cb;
}

.balance-indicator.unbalanced {
    background-color: #f8d7da;
    color: #721c24;
    border: 2px solid #f5c6cb;
}

.account-info {
    display: flex;
    flex-direction: column;
}

.account-name {
    font-size: 0.9rem;
    color: #666;
    margin-top: 0.25rem;
}

.balance-sheet-sections,
.income-statement-sections {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
    margin-bottom: 2rem;
}

.balance-sheet-section,
.income-section,
.expense-section {
    background-color: #f8f9fa;
    padding: 1.5rem;
    border-radius: 8px;
    border: 1px solid #e1e8ed;
}

.balance-sheet-section h4,
.income-section h4,
.expense-section h4 {
    margin: 0 0 1rem 0;
    color: #2c3e50;
    border-bottom: 2px solid #3498db;
    padding-bottom: 0.5rem;
}

.section-total {
    background-color: #e9ecef;
    border-top: 2px solid #2c3e50;
}

.balance-verification,
.net-income-section {
    background-color: #f8f9fa;
    padding: 1.5rem;
    border-radius: 8px;
    border: 2px solid #2c3e50;
    margin-top: 1rem;
}

.verification-row {
    border-bottom: 1px solid #dee2e6;
}

.balance-check.balanced {
    background-color: #d4edda;
    color: #155724;
}

.balance-check.unbalanced {
    background-color: #f8d7da;
    color: #721c24;
}

.net-income.profit {
    background-color: #d4edda;
    color: #155724;
}

.net-income.loss {
    background-color: #f8d7da;
    color: #721c24;
}

.summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
}

.summary-card {
    background-color: white;
    padding: 1.5rem;
    border-radius: 8px;
    border-left: 4px solid #3498db;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.summary-card h4 {
    margin: 0 0 1rem 0;
    color: #2c3e50;
    font-size: 1rem;
}

.summary-amount {
    font-size: 1.5rem;
    font-weight: 600;
    font-family: 'Courier New', monospace;
    color: #2c3e50;
}

.summary-card.assets {
    border-left-color: #3498db;
}

.summary-card.liabilities {
    border-left-color: #e74c3c;
}

.summary-card.equity {
    border-left-color: #27ae60;
}

.summary-card.revenue {
    border-left-color: #f39c12;
}

.summary-card.expenses {
    border-left-color: #9b59b6;
}

.summary-card.net-income.profit {
    border-left-color: #27ae60;
    background-color: #f8fff9;
}

.summary-card.net-income.loss {
    border-left-color: #e74c3c;
    background-color: #fff8f8;
}

.accounting-equation {
    background-color: #f8f9fa;
    padding: 2rem;
    border-radius: 8px;
    border: 1px solid #e1e8ed;
}

.accounting-equation h4 {
    margin: 0 0 1.5rem 0;
    color: #2c3e50;
    text-align: center;
}

.equation {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 1rem;
}

.equation-part {
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: white;
    padding: 1rem;
    border-radius: 8px;
    border: 1px solid #dee2e6;
    min-width: 120px;
}

.equation-part .label {
    font-size: 0.9rem;
    color: #666;
    margin-bottom: 0.5rem;
}

.equation-part .amount {
    font-size: 1.1rem;
    font-weight: 600;
    font-family: 'Courier New', monospace;
    color: #2c3e50;
}

.equation-operator {
    font-size: 1.5rem;
    font-weight: bold;
    color: #2c3e50;
}

.equation-status {
    text-align: center;
    padding: 1rem;
    border-radius: 8px;
    font-weight: 600;
    font-size: 1.1rem;
}

.equation-status.balanced {
    background-color: #d4edda;
    color: #155724;
}

.equation-status.unbalanced {
    background-color: #f8d7da;
    color: #721c24;
}

.amount.revenue {
    color: #27ae60;
}

.amount.expense {
    color: #e74c3c;
}

/* Responsive design for reports */
@media (max-width: 768px) {
    .balance-sheet-sections,
    .income-statement-sections {
        grid-template-columns: 1fr;
    }
    
    .summary-cards {
        grid-template-columns: 1fr;
    }
    
    .equation {
        flex-direction: column;
    }
    
    .equation-operator {
        transform: rotate(90deg);
    }
}
</style>
`;

document.head.insertAdjacentHTML('beforeend', reportsStyles);