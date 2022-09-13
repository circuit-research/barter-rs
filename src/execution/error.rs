use thiserror::Error;

/// All errors generated in the Barter execution module.
#[derive(Error, Copy, Clone, Debug)]
pub enum ExecutionError {
    #[error("Failed to build due to missing attribute: {0}")]
    BuilderIncomplete(&'static str),
}
