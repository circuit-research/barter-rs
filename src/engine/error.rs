use crate::{
    data::error::DataError,
};
use barter_execution::error::ExecutionError;
use thiserror::Error;

/// All errors generated in the Barter `Engine` module.
#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Data: {0}")]
    Data(#[from] DataError),

    #[error("Data: {0}")]
    Execution(#[from] ExecutionError),
}