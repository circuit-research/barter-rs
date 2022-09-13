use thiserror::Error;

// Todo: Each ExchangeClient method would have it's own variant.

/// All errors generated in the Barter execution module.
#[derive(Error, Copy, Clone, Debug)]
pub enum ExecutionError {
    #[error("Failed to build due to missing attribute: {0}")]
    BuilderIncomplete(&'static str),
}
