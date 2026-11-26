use std::{fmt, io};

#[derive(Debug)]
pub enum AppError {
    Database(rusqlite::Error),
    Filesystem { context: String, source: io::Error },
    Serialization(serde_json::Error),
    Validation(String),
    NotFound(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn filesystem(context: impl Into<String>, source: io::Error) -> Self {
        Self::Filesystem {
            context: context.into(),
            source,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {e}"),
            Self::Filesystem { context, source } => write!(f, "{context}: {source}"),
            Self::Serialization(e) => write!(f, "Could not save problem metadata: {e}"),
            Self::Validation(message) | Self::NotFound(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for AppError {}
impl From<rusqlite::Error> for AppError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Database(value)
    }
}
impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}
