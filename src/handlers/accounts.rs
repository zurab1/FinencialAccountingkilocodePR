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
pub struct ListAccountsQuery {
    pub account_type: Option<AccountType>,
}

pub async fn list_accounts(
    Extension(state): Extension<AppState>,
    Query(query): Query<ListAccountsQuery>,
) -> Result<Json<Vec<Account>>, ApiError> {
    let mut accounts = state.database.list_accounts().await?;

    // Filter by account type if specified
    if let Some(account_type) = query.account_type {
        accounts.retain(|account| account.account_type == account_type);
    }

    Ok(Json(accounts))
}

pub async fn get_account(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Account>, ApiError> {
    let account = state.database.get_account(id).await?
        .ok_or_else(|| not_found_error("Account"))?;

    Ok(Json(account))
}

pub async fn create_account(
    Extension(state): Extension<AppState>,
    Json(request): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<Account>), ApiError> {
    // Validate account code format (basic validation)
    if request.code.is_empty() {
        return Err(validation_error("Account code cannot be empty"));
    }

    if request.name.is_empty() {
        return Err(validation_error("Account name cannot be empty"));
    }

    // Check if account code already exists
    if let Some(_) = state.database.get_account_by_code(&request.code).await? {
        return Err(validation_error("Account code already exists"));
    }

    // Validate parent account exists if specified
    if let Some(parent_id) = request.parent_id {
        if state.database.get_account(parent_id).await?.is_none() {
            return Err(validation_error("Parent account does not exist"));
        }
    }

    let account = state.database.create_account(request).await?;
    Ok((StatusCode::CREATED, Json(account)))
}

pub async fn update_account(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateAccountRequest>,
) -> Result<Json<Account>, ApiError> {
    // Check if account exists
    if state.database.get_account(id).await?.is_none() {
        return Err(not_found_error("Account"));
    }

    // Validate name if provided
    if let Some(ref name) = request.name {
        if name.is_empty() {
            return Err(validation_error("Account name cannot be empty"));
        }
    }

    // Validate parent account exists if specified
    if let Some(parent_id) = request.parent_id {
        if state.database.get_account(parent_id).await?.is_none() {
            return Err(validation_error("Parent account does not exist"));
        }
        
        // Prevent circular references (account cannot be its own parent)
        if parent_id == id {
            return Err(validation_error("Account cannot be its own parent"));
        }
    }

    let account = state.database.update_account(id, request).await?
        .ok_or_else(|| not_found_error("Account"))?;

    Ok(Json(account))
}

pub async fn delete_account(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    // Check if account exists
    if state.database.get_account(id).await?.is_none() {
        return Err(not_found_error("Account"));
    }

    // TODO: Add validation to prevent deletion of accounts with transactions
    // This would require additional database queries to check for journal entries

    let deleted = state.database.delete_account(id).await?;
    
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(not_found_error("Account"))
    }
}