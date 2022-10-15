use std::collections::HashMap;
use barter_integration::model::{Exchange, Instrument};
use self::{
    consume::Consume,
    terminate::Terminate,
};
use crate::engine::{Engine, Trader};
use crate::engine::error::EngineError;

pub mod consume;
pub mod market;
pub mod order;
pub mod account;
pub mod command;
pub mod terminate;

/// [`Initialise`] can transition to one of:
/// a) [`Consumer`]
/// b) [`Terminate`]
pub struct Initialise {
    pub instruments: HashMap<Exchange, Vec<Instrument>>,
}

impl<Strategy> Trader<Strategy, Initialise> {
    pub fn init(self) -> Engine<Strategy> {
        // Send ExecutionRequests

        // Wait for response AccountEvents w/ timeout

        // Construct Accounts

        //


        Engine::Consume(Trader::from(self))
    }
}

/// a) Initialise -> Consume
impl<Strategy> From<Trader<Strategy, Initialise>> for Trader<Strategy, Consume> {
    fn from(trader: Trader<Strategy, Initialise>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Consume {

            }
        }
    }
}


/// b) Initialise -> Terminate
impl<Strategy> From<(Trader<Strategy, Initialise>, EngineError)> for Trader<Strategy, Terminate> {
    fn from((trader, error): (Trader<Strategy, Initialise>, EngineError)) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Terminate {
                reason: Err(error)
            }
        }
    }
}
