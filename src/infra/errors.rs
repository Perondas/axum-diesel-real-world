use std::fmt;

#[derive(Debug)]
pub enum InfraError {
    InternalServerError,
    NotFound,
}

impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfraError::NotFound => write!(f, "Not found"),
            InfraError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl From<diesel::result::Error> for InfraError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => InfraError::NotFound,
            _ => InfraError::InternalServerError,
        }
    }
}

impl From<deadpool_diesel::PoolError> for InfraError {
    fn from(_: deadpool_diesel::PoolError) -> Self {
        InfraError::InternalServerError
    }
}

impl From<deadpool_diesel::InteractError> for InfraError {
    fn from(_: deadpool_diesel::InteractError) -> Self {
        InfraError::InternalServerError
    }
}
