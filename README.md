# Financial Accounting System

A comprehensive double-entry bookkeeping system built with Rust and modern web technologies.

## Features

- **Double-Entry Bookkeeping**: Ensures all transactions balance (debits = credits)
- **Chart of Accounts**: Hierarchical account structure with five main types
- **Transaction Management**: Create, view, and manage financial transactions
- **Financial Reports**: Trial balance, balance sheet, income statement, and account summaries
- **Web Interface**: Modern, responsive HTML/CSS/JavaScript frontend
- **SQLite Database**: Reliable local database with automatic migrations
- **Real-time Validation**: Instant feedback on transaction balance and data integrity

## Architecture

### Backend (Rust)
- **Axum**: Modern async web framework
- **SQLx**: Type-safe SQL with compile-time verification
- **SQLite**: Embedded database with ACID compliance
- **Rust Decimal**: Precise decimal arithmetic for financial calculations
- **Chrono**: Date and time handling

### Frontend
- **HTML5**: Semantic markup with accessibility features
- **CSS3**: Professional styling with responsive design
- **Vanilla JavaScript**: Dynamic interactions without framework dependencies

### Database Schema
- **Accounts**: Chart of accounts with hierarchical structure
- **Transactions**: Financial transaction headers
- **Journal Entries**: Individual debit/credit entries for each transaction
- **Automatic Triggers**: Real-time balance updates

## Account Types

1. **Assets** (Normal Debit Balance)
   - Current Assets: Cash, Accounts Receivable, Inventory
   - Fixed Assets: Equipment, Buildings

2. **Liabilities** (Normal Credit Balance)
   - Current Liabilities: Accounts Payable, Short-term Loans
   - Long-term Liabilities: Long-term Debt

3. **Equity** (Normal Credit Balance)
   - Owner's Equity, Retained Earnings

4. **Revenue** (Normal Credit Balance)
   - Sales Revenue, Service Revenue

5. **Expenses** (Normal Debit Balance)
   - Cost of Goods Sold, Operating Expenses

## Installation

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git

### Setup
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd financial-accounting
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Set up environment variables (optional):
   ```bash
   cp .env.example .env
   # Edit .env if needed - defaults to sqlite:accounting.db
   ```

4. Run the application:
   ```bash
   cargo run
   ```

5. Open your browser and navigate to:
   ```
   http://127.0.0.1:3000
   ```

## Usage

### Getting Started

1. **Dashboard**: Overview of account balances and recent transactions
2. **Accounts**: Manage your chart of accounts
3. **Transactions**: Record financial transactions
4. **Reports**: Generate financial reports

### Creating Accounts

1. Navigate to the Accounts page
2. Click "New Account"
3. Enter account code (e.g., "1110" for Cash)
4. Enter account name
5. Select account type
6. Click "Create Account"

### Recording Transactions

1. Navigate to the Transactions page
2. Click "New Transaction"
3. Enter transaction description and date
4. Add journal entries:
   - Select account
   - Enter either debit OR credit amount
   - Add description (optional)
5. Ensure total debits equal total credits
6. Click "Create Transaction"

### Example Transaction: Cash Sale

**Description**: Cash sale of goods
**Date**: Today

**Journal Entries**:
- Cash (1110): Debit $1,000
- Sales Revenue (4100): Credit $1,000

This transaction increases both Cash (asset) and Sales Revenue, maintaining the accounting equation.

## API Endpoints

### Accounts
- `GET /api/accounts` - List all accounts
- `POST /api/accounts` - Create new account
- `GET /api/accounts/:id` - Get account by ID
- `PUT /api/accounts/:id` - Update account
- `DELETE /api/accounts/:id` - Delete account

### Transactions
- `GET /api/transactions` - List transactions (with filters)
- `POST /api/transactions` - Create new transaction
- `GET /api/transactions/:id` - Get transaction by ID
- `POST /api/transactions/validate` - Validate transaction before creation

### Reports
- `GET /api/reports/summary` - Account summary
- `GET /api/reports/trial-balance` - Trial balance
- `GET /api/reports/balance-sheet` - Balance sheet
- `GET /api/reports/income-statement` - Income statement

## Database

The system uses SQLite with the following key features:

- **ACID Compliance**: Ensures data integrity
- **Automatic Migrations**: Database schema updates automatically
- **Triggers**: Real-time balance calculations
- **Constraints**: Prevents invalid data entry

### Sample Data

The system includes a default chart of accounts with common business accounts. You can modify or extend this as needed.

## Development

### Project Structure
```
financial-accounting/
├── src/
│   ├── main.rs              # Application entry point
│   ├── models/              # Data models
│   │   ├── account.rs       # Account model and types
│   │   ├── transaction.rs   # Transaction models
│   │   └── journal_entry.rs # Journal entry models
│   ├── handlers/            # HTTP request handlers
│   │   ├── accounts.rs      # Account endpoints
│   │   ├── transactions.rs  # Transaction endpoints
│   │   ├── reports.rs       # Report endpoints
│   │   └── web.rs          # Web page handlers
│   └── database/            # Database operations
│       └── mod.rs          # Database connection and queries
├── migrations/              # Database migrations
│   └── 001_initial_schema.sql
├── static/                  # Static web assets
│   ├── css/
│   │   └── styles.css      # Application styles
│   └── js/                 # JavaScript files
│       ├── dashboard.js    # Dashboard functionality
│       ├── accounts.js     # Account management
│       ├── transactions.js # Transaction management
│       ├── reports.js      # Report generation
│       └── trial-balance.js # Trial balance page
├── Cargo.toml              # Rust dependencies
├── .env                    # Environment variables
└── README.md              # This file
```

### Running Tests
```bash
cargo test
```

### Building for Production
```bash
cargo build --release
```

## Accounting Principles

This system follows standard accounting principles:

### Double-Entry Bookkeeping
Every transaction affects at least two accounts, with total debits equaling total credits.

### Accounting Equation
**Assets = Liabilities + Equity**

The system maintains this fundamental equation at all times.

### Account Normal Balances
- **Assets & Expenses**: Normal debit balance (increase with debits)
- **Liabilities, Equity & Revenue**: Normal credit balance (increase with credits)

### Financial Statements
- **Balance Sheet**: Assets, Liabilities, and Equity at a point in time
- **Income Statement**: Revenue and Expenses over a period
- **Trial Balance**: All account balances to verify debits = credits

## Security Considerations

- Input validation on all forms
- SQL injection prevention through parameterized queries
- XSS prevention through proper HTML escaping
- CSRF protection (recommended for production)

## Limitations

- Single-user system (no authentication/authorization)
- SQLite database (not suitable for high-concurrency)
- Basic reporting (no advanced analytics)
- No audit trail (recommended for production)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions or issues, please create an issue in the repository.# FinencialAccountingkilocodePR
# FinencialAccountingkilocodePR
# FinencialAccountingkilocodePR
