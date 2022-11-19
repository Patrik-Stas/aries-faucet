use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// error format "code#message"
#[derive(Error, Debug)]
pub enum BusinessError {
    #[error("10003# Resource {0} not found")]
    ResourceNotFound(String),
    #[error("10001# Validation error on field: {0}")]
    ValidationError(String),
    #[error("10002# Argument error")]
    ArgumentError,
    #[error("10000# An internal error occurred. Please try again later.")]
    InternalError {
        #[from]
        #[source]
        source: anyhow::Error
    },
}

impl BusinessError {
    pub fn to_code(&self) -> i32 {
        let code = &self.to_string()[0..5];
        code.parse().unwrap_or(-1)
    }

    pub fn to_message(&self) -> String {
        match self {
            BusinessError::InternalError { source } => {
                source.to_string()
            },
            _ => {
                self.to_string()[7..].to_owned()
            }
        }
    }
}


impl From<mongodb::error::Error> for BusinessError {
    fn from(e: mongodb::error::Error) -> Self {
        // log::error!("mongodb error, {}", e.to_string());
        BusinessError::InternalError { source: anyhow!(e) }
    }
}

pub type BusinessResult<T> = Result<T, BusinessError>;