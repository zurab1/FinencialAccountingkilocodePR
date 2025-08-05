// Accounts page functionality
let accounts = [];

document.addEventListener('DOMContentLoaded', function() {
    loadAccounts();
    setupNewAccountForm();
});

async function loadAccounts() {
    try {
        const response = await fetch('/api/accounts');
        accounts = await response.json();
        displayAccounts(accounts);
    } catch (error) {
        console.error('Error loading accounts:', error);
        document.getElementById('accounts-table').innerHTML = 
            '<div class="error">Failed to load accounts</div>';
    }
}

function displayAccounts(accountsToShow) {
    if (accountsToShow.length === 0) {
        document.getElementById('accounts-table').innerHTML = 
            '<div class="no-data">No accounts found</div>';
        return;
    }

    const tableHtml = `
        <table class="table">
            <thead>
                <tr>
                    <th>Code</th>
                    <th>Name</th>
                    <th>Type</th>
                    <th>Balance</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                ${accountsToShow.map(account => `
                    <tr>
                        <td><strong>${escapeHtml(account.code)}</strong></td>
                        <td>${escapeHtml(account.name)}</td>
                        <td>
                            <span class="account-type ${account.account_type}">
                                ${account.account_type}
                            </span>
                        </td>
                        <td class="amount ${getBalanceClass(account)}">
                            $${formatAmount(account.balance)}
                        </td>
                        <td>
                            <button class="btn btn-secondary btn-sm" onclick="editAccount(${account.id})">
                                Edit
                            </button>
                            <button class="btn btn-danger btn-sm" onclick="deleteAccount(${account.id})">
                                Delete
                            </button>
                        </td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;

    document.getElementById('accounts-table').innerHTML = tableHtml;
}

function filterAccounts() {
    const typeFilter = document.getElementById('account-type-filter').value;
    
    let filteredAccounts = accounts;
    
    if (typeFilter) {
        filteredAccounts = accounts.filter(account => account.account_type === typeFilter);
    }
    
    displayAccounts(filteredAccounts);
}

function getBalanceClass(account) {
    if (account.balance > 0) {
        return account.account_type === 'asset' || account.account_type === 'expense' ? 'debit' : 'credit';
    } else if (account.balance < 0) {
        return account.account_type === 'asset' || account.account_type === 'expense' ? 'credit' : 'debit';
    }
    return '';
}

function showNewAccountForm() {
    document.getElementById('new-account-modal').style.display = 'block';
    document.getElementById('account-code').focus();
}

function hideNewAccountForm() {
    document.getElementById('new-account-modal').style.display = 'none';
    document.getElementById('new-account-form').reset();
}

function setupNewAccountForm() {
    const form = document.getElementById('new-account-form');
    form.addEventListener('submit', async function(e) {
        e.preventDefault();
        
        const formData = new FormData(form);
        const accountData = {
            code: formData.get('code'),
            name: formData.get('name'),
            account_type: formData.get('account_type'),
            parent_id: null // For now, we don't support parent accounts in the UI
        };

        try {
            const response = await fetch('/api/accounts', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(accountData)
            });

            if (response.ok) {
                const newAccount = await response.json();
                accounts.push(newAccount);
                displayAccounts(accounts);
                hideNewAccountForm();
                showNotification('Account created successfully', 'success');
            } else {
                const error = await response.json();
                showNotification(error.error || 'Failed to create account', 'error');
            }
        } catch (error) {
            console.error('Error creating account:', error);
            showNotification('Failed to create account', 'error');
        }
    });
}

async function editAccount(accountId) {
    // For now, we'll just show an alert. In a full implementation,
    // we would show an edit modal similar to the new account modal
    const account = accounts.find(a => a.id === accountId);
    if (account) {
        const newName = prompt('Enter new account name:', account.name);
        if (newName && newName !== account.name) {
            try {
                const response = await fetch(`/api/accounts/${accountId}`, {
                    method: 'PUT',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ name: newName })
                });

                if (response.ok) {
                    const updatedAccount = await response.json();
                    const index = accounts.findIndex(a => a.id === accountId);
                    accounts[index] = updatedAccount;
                    displayAccounts(accounts);
                    showNotification('Account updated successfully', 'success');
                } else {
                    const error = await response.json();
                    showNotification(error.error || 'Failed to update account', 'error');
                }
            } catch (error) {
                console.error('Error updating account:', error);
                showNotification('Failed to update account', 'error');
            }
        }
    }
}

async function deleteAccount(accountId) {
    const account = accounts.find(a => a.id === accountId);
    if (account && confirm(`Are you sure you want to delete account "${account.name}"?`)) {
        try {
            const response = await fetch(`/api/accounts/${accountId}`, {
                method: 'DELETE'
            });

            if (response.ok) {
                accounts = accounts.filter(a => a.id !== accountId);
                displayAccounts(accounts);
                showNotification('Account deleted successfully', 'success');
            } else {
                const error = await response.json();
                showNotification(error.error || 'Failed to delete account', 'error');
            }
        } catch (error) {
            console.error('Error deleting account:', error);
            showNotification('Failed to delete account', 'error');
        }
    }
}

// Utility functions
function formatAmount(amount) {
    return Math.abs(parseFloat(amount)).toLocaleString('en-US', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
    });
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function showNotification(message, type) {
    // Create notification element
    const notification = document.createElement('div');
    notification.className = `notification notification-${type}`;
    notification.textContent = message;
    
    // Add to page
    document.body.appendChild(notification);
    
    // Remove after 3 seconds
    setTimeout(() => {
        notification.remove();
    }, 3000);
}

// Close modal when clicking outside
window.addEventListener('click', function(event) {
    const modal = document.getElementById('new-account-modal');
    if (event.target === modal) {
        hideNewAccountForm();
    }
});

// Add notification styles
const notificationStyles = `
<style>
.btn-sm {
    padding: 0.375rem 0.75rem;
    font-size: 0.8rem;
}

.notification {
    position: fixed;
    top: 20px;
    right: 20px;
    padding: 1rem 1.5rem;
    border-radius: 4px;
    color: white;
    font-weight: 500;
    z-index: 1001;
    animation: slideIn 0.3s ease-out;
}

.notification-success {
    background-color: #27ae60;
}

.notification-error {
    background-color: #e74c3c;
}

@keyframes slideIn {
    from {
        transform: translateX(100%);
        opacity: 0;
    }
    to {
        transform: translateX(0);
        opacity: 1;
    }
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

document.head.insertAdjacentHTML('beforeend', notificationStyles);