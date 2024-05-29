use crate::{domain::models::post::PostError, infra::errors::InfraError};

pub mod posts;

impl From<InfraError> for PostError {
    fn from(value: InfraError) -> Self {
        match value {
            InfraError::NotFound => PostError::NotFound,
            InfraError::InternalServerError => PostError::InternalServerError,
        }
    }
}
