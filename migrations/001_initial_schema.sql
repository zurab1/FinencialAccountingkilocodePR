-- Create accounts table
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    account_type TEXT NOT NULL CHECK (account_type IN ('asset', 'liability', 'equity', 'revenue', 'expense')),
    parent_id INTEGER,
    balance REAL NOT NULL DEFAULT 0.0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES accounts(id)
);

-- Create transactions table
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    reference TEXT,
    transaction_date DATE NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create journal_entries table
CREATE TABLE journal_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    debit_amount REAL NOT NULL DEFAULT 0.0,
    credit_amount REAL NOT NULL DEFAULT 0.0,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE CASCADE,
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    CHECK (debit_amount >= 0 AND credit_amount >= 0),
    CHECK (NOT (debit_amount > 0 AND credit_amount > 0))
);

-- Create indexes for better performance
CREATE INDEX idx_accounts_code ON accounts(code);
CREATE INDEX idx_accounts_type ON accounts(account_type);
CREATE INDEX idx_transactions_date ON transactions(transaction_date);
CREATE INDEX idx_journal_entries_transaction ON journal_entries(transaction_id);
CREATE INDEX idx_journal_entries_account ON journal_entries(account_id);

-- Create trigger to update account balances
CREATE TRIGGER update_account_balance_insert
    AFTER INSERT ON journal_entries
BEGIN
    UPDATE accounts 
    SET balance = balance + NEW.debit_amount - NEW.credit_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.account_id;
END;

CREATE TRIGGER update_account_balance_update
    AFTER UPDATE ON journal_entries
BEGIN
    -- Reverse old entry
    UPDATE accounts 
    SET balance = balance - OLD.debit_amount + OLD.credit_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.account_id;
    
    -- Apply new entry
    UPDATE accounts 
    SET balance = balance + NEW.debit_amount - NEW.credit_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.account_id;
END;

CREATE TRIGGER update_account_balance_delete
    AFTER DELETE ON journal_entries
BEGIN
    UPDATE accounts 
    SET balance = balance - OLD.debit_amount + OLD.credit_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.account_id;
END;

-- Insert default chart of accounts
INSERT INTO accounts (code, name, account_type) VALUES
-- Assets
('1000', 'Assets', 'asset'),
('1100', 'Current Assets', 'asset'),
('1110', 'Cash', 'asset'),
('1120', 'Accounts Receivable', 'asset'),
('1130', 'Inventory', 'asset'),
('1200', 'Fixed Assets', 'asset'),
('1210', 'Equipment', 'asset'),
('1220', 'Buildings', 'asset'),

-- Liabilities
('2000', 'Liabilities', 'liability'),
('2100', 'Current Liabilities', 'liability'),
('2110', 'Accounts Payable', 'liability'),
('2120', 'Short-term Loans', 'liability'),
('2200', 'Long-term Liabilities', 'liability'),
('2210', 'Long-term Debt', 'liability'),

-- Equity
('3000', 'Equity', 'equity'),
('3100', 'Owner''s Equity', 'equity'),
('3200', 'Retained Earnings', 'equity'),

-- Revenue
('4000', 'Revenue', 'revenue'),
('4100', 'Sales Revenue', 'revenue'),
('4200', 'Service Revenue', 'revenue'),

-- Expenses
('5000', 'Expenses', 'expense'),
('5100', 'Cost of Goods Sold', 'expense'),
('5200', 'Operating Expenses', 'expense'),
('5210', 'Rent Expense', 'expense'),
('5220', 'Utilities Expense', 'expense'),
('5230', 'Office Supplies', 'expense');