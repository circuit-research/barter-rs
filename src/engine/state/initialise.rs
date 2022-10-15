use super::{
    consume::Consume,
    terminate::Terminate,
};
use crate::{
    engine::{
        Engine, Trader,
    },
    portfolio::{AccountUpdater, Initialiser, MarketUpdater},
    strategy::OrderGenerator,
};
use barter_integration::model::{Exchange, Instrument};
use std::{
    collections::HashMap,
    marker::PhantomData,
};

/// [`Initialise`] can transition to one of:
/// a) [`Consumer`]
/// b) [`Terminate`]
pub struct Initialise<Portfolio> {
    pub instruments: HashMap<Exchange, Vec<Instrument>>,
    pub phantom: PhantomData<Portfolio>,
}

impl<Strategy, Portfolio> Trader<Strategy, Initialise<Portfolio>>
where
    Strategy: OrderGenerator,
    Portfolio: Initialiser<Output = Portfolio> + MarketUpdater + AccountUpdater,
{
    pub fn init(self) -> Engine<Strategy, Portfolio> {
        // De-structure Self to access attributes required for Portfolio Initialiser
        let Self {
            mut feed,
            strategy,
            execution_tx,
            state: Initialise { instruments, .. },
        } = self;

        match Portfolio::init(instruments, &execution_tx, &mut feed) {
            // a) Initialise -> Consume
            Ok(portfolio) => {
                // Transition Engine state to Consume
                Engine::Consume(Trader {
                    feed,
                    strategy,
                    execution_tx,
                    state: Consume {
                        portfolio
                    }
                })
            }
            // b) Initialise -> Terminate
            Err(error) => {
                // Transition Engine state to Terminate
                Engine::Terminate(Trader {
                    feed,
                    strategy,
                    execution_tx,
                    state: Terminate {
                        portfolio: None,
                        reason: Err(error)
                    }
                })
            }
        }
    }
}
