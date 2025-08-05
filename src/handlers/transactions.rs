use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    models::*,
    database::Database,
    handlers::{ApiError, validation_error, not_found_error},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListTransactionsQuery {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub account_id: Option<i64>,
    pub description_contains: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl From<ListTransactionsQuery> for TransactionFilter {
    fn from(query: ListTransactionsQuery) -> Self {
        TransactionFilter {
            start_date: query.start_date,
            end_date: query.end_date,
            account_id: query.account_id,
            description_contains: query.description_contains,
            min_amount: None,
            max_amount: None,
            limit: query.limit.or(Some(50)),
            offset: query.offset.or(Some(0)),
        }
    }
}

pub async fn list_transactions(
    Extension(state): Extension<AppState>,
    Query(query): Query<ListTransactionsQuery>,
) -> Result<Json<Vec<TransactionWithEntries>>, ApiError> {
    let filter = TransactionFilter::from(query);
    let transactions = state.database.list_transactions(filter).await?;
    Ok(Json(transactions))
}

pub async fn get_transaction(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<TransactionWithEntries>, ApiError> {
    let transaction = state.database.get_transaction(id).await?
        .ok_or_else(|| not_found_error("Transaction"))?;

    Ok(Json(transaction))
}

pub async fn create_transaction(
    Extension(state): Extension<AppState>,
    Json(request): Json<CreateTransactionRequest>,
) -> Result<(StatusCode, Json<TransactionWithEntries>), ApiError> {
    // Validate the transaction request
    if let Err(validation_err) = request.validate() {
        return Err(validation_error(&validation_err.to_string()));
    }

    // Validate that all referenced accounts exist
    for entry in &request.journal_entries {
        if state.database.get_account(entry.account_id).await?.is_none() {
            return Err(validation_error(&format!(
                "Account with ID {} does not exist", 
                entry.account_id
            )));
        }
    }

    // Validate individual journal entries
    for (index, entry) in request.journal_entries.iter().enumerate() {
        if let Err(validation_msg) = entry.validate() {
            return Err(validation_error(&format!(
                "Journal entry {}: {}", 
                index + 1, 
                validation_msg
            )));
        }
    }

    let transaction = state.database.create_transaction(request).await?;
    Ok((StatusCode::CREATED, Json(transaction)))
}

pub async fn update_transaction(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateTransactionRequest>,
) -> Result<Json<TransactionWithEntries>, ApiError> {
    // Check if transaction exists
    if state.database.get_transaction(id).await?.is_none() {
        return Err(not_found_error("Transaction"));
    }

    // Validate description if provided
    if let Some(ref description) = request.description {
        if description.is_empty() {
            return Err(validation_error("Transaction description cannot be empty"));
        }
    }

    // TODO: Implement transaction update logic
    // This is more complex because it involves updating journal entries
    // For now, we'll return an error indicating this feature is not implemented
    Err(ApiError {
        status: StatusCode::NOT_IMPLEMENTED,
        message: "Transaction updates are not yet implemented".to_string(),
    })
}

pub async fn delete_transaction(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    // Check if transaction exists
    if state.database.get_transaction(id).await?.is_none() {
        return Err(not_found_error("Transaction"));
    }

    // TODO: Implement transaction deletion logic
    // This involves deleting all associated journal entries and updating account balances
    // For now, we'll return an error indicating this feature is not implemented
    Err(ApiError {
        status: StatusCode::NOT_IMPLEMENTED,
        message: "Transaction deletion is not yet implemented".to_string(),
    })
}

// Helper endpoint to validate a transaction before creating it
pub async fn validate_transaction(
    Extension(state): Extension<AppState>,
    Json(request): Json<CreateTransactionRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate the transaction request
    if let Err(validation_err) = request.validate() {
        return Ok(Json(serde_json::json!({
            "valid": false,
            "errors": [validation_err.to_string()]
        })));
    }

    let mut errors = Vec::new();

    // Validate that all referenced accounts exist
    for entry in &request.journal_entries {
        if state.database.get_account(entry.account_id).await?.is_none() {
            errors.push(format!(
                "Account with ID {} does not exist", 
                entry.account_id
            ));
        }
    }

    // Validate individual journal entries
    for (index, entry) in request.journal_entries.iter().enumerate() {
        if let Err(validation_msg) = entry.validate() {
            errors.push(format!(
                "Journal entry {}: {}", 
                index + 1, 
                validation_msg
            ));
        }
    }

    let is_valid = errors.is_empty();
    let total_amount = if is_valid { Some(request.total_amount()) } else { None };

    Ok(Json(serde_json::json!({
        "valid": is_valid,
        "errors": errors,
        "total_amount": total_amount
    })))
}