use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Error, Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[serde(tag = "kind", content = "details", rename_all = "snake_case")]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Entity not found: {entity_type} with ID '{id}'")]
    NotFound {
        entity_type: String,
        id: String,
    },

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Invalid state transition: {from} -> {to} ({reason})")]
    InvalidStateTransition {
        from: String,
        to: String,
        reason: String,
    },

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Security violation: {0}")]
    Security(String),

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Internal domain error: {0}")]
    Internal(String),
}
