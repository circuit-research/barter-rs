use crate::engine::{
    error::EngineError,
};

/// [`Terminate`] cannot transition to another state.
pub struct Terminate<Portfolio> {
    pub portfolio: Option<Portfolio>,
    pub reason: Result<&'static str, EngineError>,
}