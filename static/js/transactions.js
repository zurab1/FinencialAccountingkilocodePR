// Transactions page functionality
let transactions = [];
let accounts = [];
let journalEntryCount = 0;

document.addEventListener('DOMContentLoaded', function() {
    loadTransactions();
    loadAccounts();
    setupNewTransactionForm();
    
    // Set default date to today
    document.getElementById('transaction-date').value = new Date().toISOString().split('T')[0];
});

async function loadTransactions() {
    try {
        const response = await fetch('/api/transactions');
        transactions = await response.json();
        displayTransactions(transactions);
    } catch (error) {
        console.error('Error loading transactions:', error);
        document.getElementById('transactions-table').innerHTML = 
            '<div class="error">Failed to load transactions</div>';
    }
}

async function loadAccounts() {
    try {
        const response = await fetch('/api/accounts');
        accounts = await response.json();
    } catch (error) {
        console.error('Error loading accounts:', error);
    }
}

function displayTransactions(transactionsToShow) {
    if (transactionsToShow.length === 0) {
        document.getElementById('transactions-table').innerHTML = 
            '<div class="no-data">No transactions found</div>';
        return;
    }

    const tableHtml = `
        <table class="table">
            <thead>
                <tr>
                    <th>Date</th>
                    <th>Description</th>
                    <th>Reference</th>
                    <th>Amount</th>
                    <th>Status</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                ${transactionsToShow.map(transaction => `
                    <tr>
                        <td>${formatDate(transaction.transaction.transaction_date)}</td>
                        <td>${escapeHtml(transaction.transaction.description)}</td>
                        <td>${transaction.transaction.reference || '-'}</td>
                        <td class="amount">$${formatAmount(transaction.net_amount())}</td>
                        <td>
                            <span class="status ${transaction.is_balanced() ? 'balanced' : 'unbalanced'}">
                                ${transaction.is_balanced() ? 'Balanced' : 'Unbalanced'}
                            </span>
                        </td>
                        <td>
                            <button class="btn btn-secondary btn-sm" onclick="viewTransaction(${transaction.transaction.id})">
                                View
                            </button>
                        </td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;

    document.getElementById('transactions-table').innerHTML = tableHtml;
}

function filterTransactions() {
    const startDate = document.getElementById('start-date').value;
    const endDate = document.getElementById('end-date').value;
    const descriptionFilter = document.getElementById('description-filter').value.toLowerCase();
    
    let filteredTransactions = transactions;
    
    if (startDate) {
        filteredTransactions = filteredTransactions.filter(t => 
            t.transaction.transaction_date >= startDate
        );
    }
    
    if (endDate) {
        filteredTransactions = filteredTransactions.filter(t => 
            t.transaction.transaction_date <= endDate
        );
    }
    
    if (descriptionFilter) {
        filteredTransactions = filteredTransactions.filter(t => 
            t.transaction.description.toLowerCase().includes(descriptionFilter)
        );
    }
    
    displayTransactions(filteredTransactions);
}

function showNewTransactionForm() {
    document.getElementById('new-transaction-modal').style.display = 'block';
    journalEntryCount = 0;
    document.getElementById('journal-entries').innerHTML = '';
    
    // Add two initial journal entries
    addJournalEntry();
    addJournalEntry();
    
    document.getElementById('transaction-description').focus();
}

function hideNewTransactionForm() {
    document.getElementById('new-transaction-modal').style.display = 'none';
    document.getElementById('new-transaction-form').reset();
    document.getElementById('journal-entries').innerHTML = '';
    journalEntryCount = 0;
}

function addJournalEntry() {
    journalEntryCount++;
    const entryHtml = `
        <div class="journal-entry" id="entry-${journalEntryCount}">
            <div class="form-group">
                <label>Account:</label>
                <select name="account_id" required onchange="updateBalanceCheck()">
                    <option value="">Select Account</option>
                    ${accounts.map(account => `
                        <option value="${account.id}">${account.code} - ${account.name}</option>
                    `).join('')}
                </select>
            </div>
            <div class="form-group">
                <label>Debit:</label>
                <input type="number" name="debit_amount" step="0.01" min="0" 
                       onchange="handleAmountChange(this)" placeholder="0.00">
            </div>
            <div class="form-group">
                <label>Credit:</label>
                <input type="number" name="credit_amount" step="0.01" min="0" 
                       onchange="handleAmountChange(this)" placeholder="0.00">
            </div>
            <div class="form-group">
                <label>Description:</label>
                <input type="text" name="entry_description" placeholder="Optional">
            </div>
            <div class="form-group">
                <button type="button" class="btn btn-danger btn-sm" onclick="removeJournalEntry(${journalEntryCount})">
                    Remove
                </button>
            </div>
        </div>
    `;
    
    document.getElementById('journal-entries').insertAdjacentHTML('beforeend', entryHtml);
}

function removeJournalEntry(entryId) {
    const entry = document.getElementById(`entry-${entryId}`);
    if (entry) {
        entry.remove();
        updateBalanceCheck();
    }
}

function handleAmountChange(input) {
    const entry = input.closest('.journal-entry');
    const debitInput = entry.querySelector('input[name="debit_amount"]');
    const creditInput = entry.querySelector('input[name="credit_amount"]');
    
    // Clear the other field when one is entered
    if (input.name === 'debit_amount' && input.value) {
        creditInput.value = '';
    } else if (input.name === 'credit_amount' && input.value) {
        debitInput.value = '';
    }
    
    updateBalanceCheck();
}

function updateBalanceCheck() {
    const entries = document.querySelectorAll('.journal-entry');
    let totalDebits = 0;
    let totalCredits = 0;
    
    entries.forEach(entry => {
        const debitAmount = parseFloat(entry.querySelector('input[name="debit_amount"]').value) || 0;
        const creditAmount = parseFloat(entry.querySelector('input[name="credit_amount"]').value) || 0;
        
        totalDebits += debitAmount;
        totalCredits += creditAmount;
    });
    
    document.getElementById('total-debits').textContent = formatAmount(totalDebits);
    document.getElementById('total-credits').textContent = formatAmount(totalCredits);
    
    const balanceStatus = document.getElementById('balance-status');
    const submitButton = document.getElementById('submit-transaction');
    
    if (totalDebits === totalCredits && totalDebits > 0) {
        balanceStatus.textContent = 'Balanced ✓';
        balanceStatus.className = 'balance-status balanced';
        submitButton.disabled = false;
    } else {
        balanceStatus.textContent = 'Unbalanced';
        balanceStatus.className = 'balance-status unbalanced';
        submitButton.disabled = true;
    }
}

function setupNewTransactionForm() {
    const form = document.getElementById('new-transaction-form');
    form.addEventListener('submit', async function(e) {
        e.preventDefault();
        
        const formData = new FormData(form);
        const entries = document.querySelectorAll('.journal-entry');
        
        const journalEntries = [];
        entries.forEach(entry => {
            const accountId = parseInt(entry.querySelector('select[name="account_id"]').value);
            const debitAmount = parseFloat(entry.querySelector('input[name="debit_amount"]').value) || null;
            const creditAmount = parseFloat(entry.querySelector('input[name="credit_amount"]').value) || null;
            const description = entry.querySelector('input[name="entry_description"]').value || null;
            
            if (accountId && (debitAmount || creditAmount)) {
                journalEntries.push({
                    account_id: accountId,
                    debit_amount: debitAmount,
                    credit_amount: creditAmount,
                    description: description
                });
            }
        });
        
        const transactionData = {
            description: formData.get('description'),
            reference: formData.get('reference') || null,
            transaction_date: formData.get('transaction_date'),
            journal_entries: journalEntries
        };

        try {
            const response = await fetch('/api/transactions', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(transactionData)
            });

            if (response.ok) {
                const newTransaction = await response.json();
                transactions.unshift(newTransaction);
                displayTransactions(transactions);
                hideNewTransactionForm();
                showNotification('Transaction created successfully', 'success');
            } else {
                const error = await response.json();
                showNotification(error.error || 'Failed to create transaction', 'error');
            }
        } catch (error) {
            console.error('Error creating transaction:', error);
            showNotification('Failed to create transaction', 'error');
        }
    });
}

async function viewTransaction(transactionId) {
    try {
        const response = await fetch(`/api/transactions/${transactionId}`);
        const transaction = await response.json();
        
        let entriesHtml = '';
        transaction.journal_entries.forEach(entry => {
            entriesHtml += `
                <tr>
                    <td>${entry.account_code}</td>
                    <td>${entry.account_name}</td>
                    <td class="amount debit">${entry.debit_amount > 0 ? '$' + formatAmount(entry.debit_amount) : '-'}</td>
                    <td class="amount credit">${entry.credit_amount > 0 ? '$' + formatAmount(entry.credit_amount) : '-'}</td>
                    <td>${entry.description || '-'}</td>
                </tr>
            `;
        });
        
        const modalHtml = `
            <div class="modal" id="view-transaction-modal">
                <div class="modal-content large">
                    <div class="modal-header">
                        <h3>Transaction Details</h3>
                        <span class="close" onclick="closeViewModal()">&times;</span>
                    </div>
                    <div style="padding: 1.5rem;">
                        <div class="transaction-details">
                            <p><strong>Description:</strong> ${escapeHtml(transaction.transaction.description)}</p>
                            <p><strong>Date:</strong> ${formatDate(transaction.transaction.transaction_date)}</p>
                            <p><strong>Reference:</strong> ${transaction.transaction.reference || 'N/A'}</p>
                            <p><strong>Status:</strong> 
                                <span class="status ${transaction.is_balanced() ? 'balanced' : 'unbalanced'}">
                                    ${transaction.is_balanced() ? 'Balanced' : 'Unbalanced'}
                                </span>
                            </p>
                        </div>
                        
                        <h4>Journal Entries</h4>
                        <table class="table">
                            <thead>
                                <tr>
                                    <th>Account Code</th>
                                    <th>Account Name</th>
                                    <th>Debit</th>
                                    <th>Credit</th>
                                    <th>Description</th>
                                </tr>
                            </thead>
                            <tbody>
                                ${entriesHtml}
                                <tr class="totals">
                                    <td colspan="2"><strong>Totals:</strong></td>
                                    <td class="amount debit"><strong>$${formatAmount(transaction.total_debits)}</strong></td>
                                    <td class="amount credit"><strong>$${formatAmount(transaction.total_credits)}</strong></td>
                                    <td></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        `;
        
        document.body.insertAdjacentHTML('beforeend', modalHtml);
        document.getElementById('view-transaction-modal').style.display = 'block';
        
    } catch (error) {
        console.error('Error loading transaction:', error);
        showNotification('Failed to load transaction details', 'error');
    }
}

function closeViewModal() {
    const modal = document.getElementById('view-transaction-modal');
    if (modal) {
        modal.remove();
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

function showNotification(message, type) {
    const notification = document.createElement('div');
    notification.className = `notification notification-${type}`;
    notification.textContent = message;
    
    document.body.appendChild(notification);
    
    setTimeout(() => {
        notification.remove();
    }, 3000);
}

// Close modal when clicking outside
window.addEventListener('click', function(event) {
    const modal = document.getElementById('new-transaction-modal');
    if (event.target === modal) {
        hideNewTransactionForm();
    }
});

// Add transaction-specific styles
const transactionStyles = `
<style>
.status {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 500;
    text-transform: uppercase;
}

.status.balanced {
    background-color: #d4edda;
    color: #155724;
}

.status.unbalanced {
    background-color: #f8d7da;
    color: #721c24;
}

.transaction-details {
    background-color: #f8f9fa;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1.5rem;
}

.transaction-details p {
    margin-bottom: 0.5rem;
}
</style>
`;

document.head.insertAdjacentHTML('beforeend', transactionStyles);