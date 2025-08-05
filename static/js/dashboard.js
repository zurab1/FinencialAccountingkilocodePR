// Dashboard functionality
document.addEventListener('DOMContentLoaded', function() {
    loadAccountSummary();
    loadRecentTransactions();
    checkBalanceStatus();
});

async function loadAccountSummary() {
    try {
        const response = await fetch('/api/reports/summary');
        const summary = await response.json();
        
        const summaryHtml = `
            <div class="summary-grid">
                <div class="summary-item">
                    <span class="summary-label">Total Assets:</span>
                    <span class="summary-value">$${formatAmount(summary.total_assets)}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">Total Liabilities:</span>
                    <span class="summary-value">$${formatAmount(summary.total_liabilities)}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">Total Equity:</span>
                    <span class="summary-value">$${formatAmount(summary.total_equity)}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">Net Income:</span>
                    <span class="summary-value ${summary.net_income >= 0 ? 'positive' : 'negative'}">
                        $${formatAmount(summary.net_income)}
                    </span>
                </div>
            </div>
        `;
        
        document.getElementById('account-summary').innerHTML = summaryHtml;
    } catch (error) {
        console.error('Error loading account summary:', error);
        document.getElementById('account-summary').innerHTML = 
            '<div class="error">Failed to load account summary</div>';
    }
}

async function loadRecentTransactions() {
    try {
        const response = await fetch('/api/transactions?limit=5');
        const transactions = await response.json();
        
        if (transactions.length === 0) {
            document.getElementById('recent-transactions').innerHTML = 
                '<div class="no-data">No transactions found</div>';
            return;
        }
        
        const transactionsHtml = `
            <div class="transactions-list">
                ${transactions.map(transaction => `
                    <div class="transaction-item">
                        <div class="transaction-info">
                            <div class="transaction-description">${escapeHtml(transaction.transaction.description)}</div>
                            <div class="transaction-date">${formatDate(transaction.transaction.transaction_date)}</div>
                        </div>
                        <div class="transaction-amount">$${formatAmount(transaction.net_amount())}</div>
                    </div>
                `).join('')}
            </div>
            <div class="view-all">
                <a href="/transactions" class="btn btn-secondary">View All Transactions</a>
            </div>
        `;
        
        document.getElementById('recent-transactions').innerHTML = transactionsHtml;
    } catch (error) {
        console.error('Error loading recent transactions:', error);
        document.getElementById('recent-transactions').innerHTML = 
            '<div class="error">Failed to load recent transactions</div>';
    }
}

async function checkBalanceStatus() {
    try {
        const response = await fetch('/api/reports/trial-balance');
        const trialBalance = await response.json();
        
        const statusElement = document.getElementById('balance-status');
        if (trialBalance.is_balanced) {
            statusElement.textContent = 'Balanced';
            statusElement.className = 'status-value status-ok';
        } else {
            statusElement.textContent = 'Unbalanced';
            statusElement.className = 'status-value status-error';
        }
    } catch (error) {
        console.error('Error checking balance status:', error);
        const statusElement = document.getElementById('balance-status');
        statusElement.textContent = 'Error';
        statusElement.className = 'status-value status-error';
    }
}

// Utility functions
function formatAmount(amount) {
    return parseFloat(amount).toLocaleString('en-US', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
    });
}

function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
    });
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Add some CSS for the dashboard-specific elements
const dashboardStyles = `
<style>
.summary-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
}

.summary-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background-color: #f8f9fa;
    border-radius: 4px;
    border-left: 4px solid #3498db;
}

.summary-label {
    font-weight: 500;
    color: #2c3e50;
}

.summary-value {
    font-weight: 600;
    font-family: 'Courier New', monospace;
}

.summary-value.positive {
    color: #27ae60;
}

.summary-value.negative {
    color: #e74c3c;
}

.transactions-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.transaction-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background-color: #f8f9fa;
    border-radius: 4px;
    border-left: 4px solid #95a5a6;
}

.transaction-description {
    font-weight: 500;
    color: #2c3e50;
}

.transaction-date {
    font-size: 0.85rem;
    color: #666;
}

.transaction-amount {
    font-weight: 600;
    font-family: 'Courier New', monospace;
    color: #2c3e50;
}

.view-all {
    margin-top: 1rem;
    text-align: center;
}

.no-data, .error {
    text-align: center;
    padding: 2rem;
    color: #666;
    font-style: italic;
}

.error {
    color: #e74c3c;
}
</style>
`;

document.head.insertAdjacentHTML('beforeend', dashboardStyles);