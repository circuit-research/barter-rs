use crate::engine::{
    error::EngineError,
};

/// [`Terminate`] cannot transition to another state.
pub struct Terminate {
    pub reason: Result<&'static str, EngineError>,
}