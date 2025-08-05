// Trial Balance page functionality
document.addEventListener('DOMContentLoaded', function() {
    loadTrialBalance();
});

async function loadTrialBalance() {
    try {
        const response = await fetch('/api/reports/trial-balance');
        const trialBalance = await response.json();
        displayTrialBalance(trialBalance);
    } catch (error) {
        console.error('Error loading trial balance:', error);
        document.getElementById('trial-balance-content').innerHTML = 
            '<div class="error">Failed to load trial balance</div>';
    }
}

function displayTrialBalance(trialBalance) {
    const currentDate = new Date().toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });

    const tableHtml = `
        <div class="report-header">
            <div class="report-title-section">
                <h3>Trial Balance</h3>
                <p class="report-date">As of ${currentDate}</p>
            </div>
            <div class="balance-indicator">
                <span class="balance-status ${trialBalance.is_balanced ? 'balanced' : 'unbalanced'}">
                    ${trialBalance.is_balanced ? 'Balanced ✓' : 'Unbalanced ⚠'}
                </span>
            </div>
        </div>
        
        <table class="table trial-balance-table">
            <thead>
                <tr>
                    <th>Account Code</th>
                    <th>Account Name</th>
                    <th>Account Type</th>
                    <th class="amount">Debit Balance</th>
                    <th class="amount">Credit Balance</th>
                </tr>
            </thead>
            <tbody>
                ${trialBalance.entries.map(entry => `
                    <tr>
                        <td><strong>${escapeHtml(entry.account_code)}</strong></td>
                        <td>${escapeHtml(entry.account_name)}</td>
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
                    <td colspan="3"><strong>TOTALS</strong></td>
                    <td class="amount debit"><strong>$${formatAmount(trialBalance.total_debits)}</strong></td>
                    <td class="amount credit"><strong>$${formatAmount(trialBalance.total_credits)}</strong></td>
                </tr>
            </tfoot>
        </table>
        
        <div class="trial-balance-summary">
            <div class="summary-section">
                <h4>Summary by Account Type</h4>
                <div class="account-type-summary">
                    ${generateAccountTypeSummary(trialBalance.entries)}
                </div>
            </div>
            
            <div class="balance-verification">
                <h4>Balance Verification</h4>
                <div class="verification-details">
                    <div class="verification-item">
                        <span class="label">Total Debits:</span>
                        <span class="value">$${formatAmount(trialBalance.total_debits)}</span>
                    </div>
                    <div class="verification-item">
                        <span class="label">Total Credits:</span>
                        <span class="value">$${formatAmount(trialBalance.total_credits)}</span>
                    </div>
                    <div class="verification-item">
                        <span class="label">Difference:</span>
                        <span class="value ${trialBalance.is_balanced ? 'balanced' : 'unbalanced'}">
                            $${formatAmount(Math.abs(trialBalance.total_debits - trialBalance.total_credits))}
                        </span>
                    </div>
                    <div class="verification-status">
                        ${trialBalance.is_balanced 
                            ? '<span class="status-message balanced">✓ Trial balance is in balance</span>'
                            : '<span class="status-message unbalanced">⚠ Trial balance is out of balance</span>'
                        }
                    </div>
                </div>
            </div>
        </div>
    `;

    document.getElementById('trial-balance-content').innerHTML = tableHtml;
}

function generateAccountTypeSummary(entries) {
    const summary = {
        asset: { debit: 0, credit: 0, count: 0 },
        liability: { debit: 0, credit: 0, count: 0 },
        equity: { debit: 0, credit: 0, count: 0 },
        revenue: { debit: 0, credit: 0, count: 0 },
        expense: { debit: 0, credit: 0, count: 0 }
    };

    entries.forEach(entry => {
        if (summary[entry.account_type]) {
            summary[entry.account_type].debit += parseFloat(entry.debit_balance);
            summary[entry.account_type].credit += parseFloat(entry.credit_balance);
            summary[entry.account_type].count++;
        }
    });

    return Object.entries(summary)
        .filter(([type, data]) => data.count > 0)
        .map(([type, data]) => `
            <div class="type-summary-item">
                <div class="type-header">
                    <span class="account-type ${type}">${capitalizeFirst(type)}</span>
                    <span class="account-count">(${data.count} accounts)</span>
                </div>
                <div class="type-amounts">
                    <span class="debit-total">Debits: $${formatAmount(data.debit)}</span>
                    <span class="credit-total">Credits: $${formatAmount(data.credit)}</span>
                    <span class="net-balance">Net: $${formatAmount(Math.abs(data.debit - data.credit))}</span>
                </div>
            </div>
        `).join('');
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

// Add trial balance specific styles
const trialBalanceStyles = `
<style>
.report-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid #2c3e50;
}

.report-title-section h3 {
    margin: 0;
    font-size: 2rem;
    color: #2c3e50;
}

.report-date {
    margin: 0.5rem 0 0 0;
    color: #666;
    font-style: italic;
}

.balance-indicator {
    text-align: right;
}

.balance-status {
    padding: 0.5rem 1rem;
    border-radius: 4px;
    font-weight: 600;
    font-size: 1rem;
}

.balance-status.balanced {
    background-color: #d4edda;
    color: #155724;
    border: 2px solid #c3e6cb;
}

.balance-status.unbalanced {
    background-color: #f8d7da;
    color: #721c24;
    border: 2px solid #f5c6cb;
}

.trial-balance-table {
    margin-bottom: 2rem;
}

.trial-balance-table th {
    background-color: #2c3e50;
    color: white;
    font-weight: 600;
    padding: 1rem 0.75rem;
}

.trial-balance-table .totals {
    background-color: #f8f9fa;
    border-top: 3px solid #2c3e50;
    font-weight: bold;
}

.trial-balance-table .totals td {
    padding: 1rem 0.75rem;
    font-size: 1.1rem;
}

.trial-balance-summary {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    margin-top: 2rem;
}

.summary-section,
.balance-verification {
    background-color: #f8f9fa;
    padding: 1.5rem;
    border-radius: 8px;
    border: 1px solid #e1e8ed;
}

.summary-section h4,
.balance-verification h4 {
    margin: 0 0 1rem 0;
    color: #2c3e50;
    border-bottom: 1px solid #dee2e6;
    padding-bottom: 0.5rem;
}

.account-type-summary {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.type-summary-item {
    background-color: white;
    padding: 1rem;
    border-radius: 4px;
    border-left: 4px solid #3498db;
}

.type-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
}

.account-count {
    font-size: 0.85rem;
    color: #666;
}

.type-amounts {
    display: flex;
    justify-content: space-between;
    font-size: 0.9rem;
}

.debit-total {
    color: #27ae60;
}

.credit-total {
    color: #e74c3c;
}

.net-balance {
    font-weight: 600;
    color: #2c3e50;
}

.verification-details {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.verification-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    background-color: white;
    border-radius: 4px;
}

.verification-item .label {
    font-weight: 500;
}

.verification-item .value {
    font-family: 'Courier New', monospace;
    font-weight: 600;
}

.verification-item .value.balanced {
    color: #27ae60;
}

.verification-item .value.unbalanced {
    color: #e74c3c;
}

.verification-status {
    margin-top: 1rem;
    padding: 1rem;
    text-align: center;
    border-radius: 4px;
}

.status-message {
    font-weight: 600;
    font-size: 1.1rem;
}

.status-message.balanced {
    color: #155724;
    background-color: #d4edda;
    padding: 0.75rem 1rem;
    border-radius: 4px;
    border: 1px solid #c3e6cb;
}

.status-message.unbalanced {
    color: #721c24;
    background-color: #f8d7da;
    padding: 0.75rem 1rem;
    border-radius: 4px;
    border: 1px solid #f5c6cb;
}

.error {
    text-align: center;
    padding: 2rem;
    color: #e74c3c;
    font-size: 1.1rem;
}

/* Responsive design for trial balance */
@media (max-width: 768px) {
    .report-header {
        flex-direction: column;
        gap: 1rem;
    }
    
    .trial-balance-summary {
        grid-template-columns: 1fr;
        gap: 1rem;
    }
    
    .type-amounts {
        flex-direction: column;
        gap: 0.25rem;
    }
    
    .trial-balance-table {
        font-size: 0.9rem;
    }
    
    .trial-balance-table th,
    .trial-balance-table td {
        padding: 0.5rem 0.25rem;
    }
}

/* Print styles for trial balance */
@media print {
    .trial-balance-summary {
        grid-template-columns: 1fr;
        page-break-inside: avoid;
    }
    
    .summary-section,
    .balance-verification {
        border: 1px solid #000;
        margin-bottom: 1rem;
    }
    
    .trial-balance-table th {
        background-color: #f0f0f0 !important;
        color: #000 !important;
    }
}
</style>
`;

document.head.insertAdjacentHTML('beforeend', trialBalanceStyles);