use self::{
    consume::Consume,
    terminate::Terminate,
};
use crate::engine::{
    Engine, Trader,
    error::EngineError,
};
use barter_integration::model::{Exchange, Instrument};
use std::collections::HashMap;
use crate::portfolio::{AccountUpdater, MarketUpdater};

pub mod consume;
pub mod market;
pub mod order;
pub mod account;
pub mod command;
pub mod terminate;

/// [`Initialise`] can transition to one of:
/// a) [`Consumer`]
/// b) [`Terminate`]
pub struct Initialise<Portfolio> {
    pub instruments: HashMap<Exchange, Vec<Instrument>>,
    pub phantom: Portfolio,
}

impl<Strategy, Portfolio> Trader<Strategy, Initialise<Portfolio>>
where
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn init(self) -> Engine<Strategy, Portfolio> {
        // Send ExecutionRequests

        // Wait for response AccountEvents w/ timeout

        // Construct Accounts

        //


        Engine::Consume(Trader::from(self))
    }
}

/// a) Initialise -> Consume
impl<Strategy, Portfolio> From<(Trader<Strategy, Initialise<Portfolio>>, Portfolio)> for Trader<Strategy, Consume<Portfolio>> {
    fn from((trader, portfolio): (Trader<Strategy, Initialise<Portfolio>>, Portfolio)) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Consume {
                portfolio
            }
        }
    }
}


/// b) Initialise -> Terminate
impl<Strategy, Portfolio> From<(Trader<Strategy, Initialise<Portfolio>>, EngineError)> for Trader<Strategy, Terminate> {
    fn from((trader, error): (Trader<Strategy, Initialise<Portfolio>>, EngineError)) -> Self {
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
