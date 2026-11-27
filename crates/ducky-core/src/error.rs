use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("{context}: {source}")]
    Filesystem {
        context: String,
        #[source]
        source: io::Error,
    },
}

pub type CoreResult<T> = Result<T, CoreError>;

impl CoreError {
    pub fn filesystem(context: impl Into<String>, source: io::Error) -> Self {
        Self::Filesystem {
            context: context.into(),
            source,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Validation(_) => "validation_error",
            Self::NotFound(_) => "not_found",
            Self::Conflict(_) => "conflict",
            Self::Database(_) | Self::Migration(_) => "database_error",
            Self::Serialization(_) => "serialization_error",
            Self::Filesystem { .. } => "filesystem_error",
        }
    }

    pub fn safe_message(&self) -> String {
        match self {
            Self::Database(_) | Self::Migration(_) => {
                "A local database operation failed".to_string()
            }
            Self::Filesystem { context, source } => format!("{context}: {}", source.kind()),
            other => other.to_string(),
        }
    }
}
