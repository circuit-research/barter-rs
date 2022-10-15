use self::{
    error::EngineError,
    state::{
        initialise::Initialise,
        account::UpdateFromAccount,
        command::ExecuteCommand,
        consume::Consume,
        market::UpdateFromMarket,
        order::{Algorithmic, GenerateOrder, Manual},
        terminate::Terminate,
    }
};
use crate::{
    event::{Command, EventFeed},
    execution::ExecutionRequest,
    portfolio::{AccountUpdater, Initialiser, MarketUpdater},
    strategy::OrderGenerator,
};
use barter_integration::model::{Exchange, Instrument};
use barter_data::model::MarketEvent;
use barter_execution::model::{
    AccountEvent,
    order::{Order, RequestOpen}
};
use std::{
    collections::HashMap,
    marker::PhantomData
};
use tokio::sync::mpsc;

pub mod state;
pub mod error;

// Todo:
//  - Should AccountEvent contain an exchange_timestamp?
//  - May benefit from having 'EngineBuilder' build all components of the system
//   '--> ie/ spawns all threads & tasks for barter-data, execution, etc
//    --> "Engine" could become "TraderStates" or similar
//  - Move MarketUpdater from somewhere general to both Strategy & Portfolio


pub enum Engine<Strategy, Portfolio>
where
    Strategy: OrderGenerator,
    Portfolio: MarketUpdater + AccountUpdater,
{
    Initialise(Trader<Strategy, Initialise<Portfolio>>),
    Consume(Trader<Strategy, Consume<Portfolio>>),
    UpdateFromMarket((Trader<Strategy, UpdateFromMarket<Portfolio>>, MarketEvent)),
    GenerateOrderAlgorithmic(Trader<Strategy, GenerateOrder<Portfolio, Algorithmic>>),
    GenerateOrderManual((Trader<Strategy, GenerateOrder<Portfolio, Manual>>, Order<RequestOpen>)),
    UpdateFromAccount((Trader<Strategy, UpdateFromAccount<Portfolio>>, AccountEvent)),
    ExecuteCommand((Trader<Strategy, ExecuteCommand<Portfolio>>, Command)),
    Terminate(Trader<Strategy, Terminate<Portfolio>>)
}

pub struct Trader<Strategy, State> {
    pub feed: EventFeed,
    pub strategy: Strategy,
    pub execution_tx: mpsc::UnboundedSender<ExecutionRequest>,
    pub state: State,
}

impl<Strategy, Portfolio> Engine<Strategy, Portfolio>
where
    Strategy: MarketUpdater + OrderGenerator,
    Portfolio: Initialiser<Output = Portfolio> + MarketUpdater + AccountUpdater
{
    /// Builder to construct [`Engine`] instances.
    pub fn builder() -> EngineBuilder<Strategy, Portfolio> {
        EngineBuilder::new()
    }

    pub fn run(mut self) {
        'trading: loop {
            // Transition to the next trading state
            self = self.next();

            if let Self::Terminate(_) = self {
                // Todo: Print trading session results & persist
                break 'trading
            }
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Initialise(trader) => {
                // Can transition to one of:
                // Consume | Terminate
                trader.init()
            }
            Self::Consume(trader) => {
                // Transitions to one of:
                // UpdateFromMarket | UpdateFromAccount | ExecuteCommand | Terminate
                trader.next_event()
            },
            Self::UpdateFromMarket((trader, market)) => {
                // Always transitions to: GenerateOrder<Algorithmic>
                trader.update(market)
            },
            Self::GenerateOrderAlgorithmic(trader) => {
                // Transitions to one of:
                // Consume | Terminate
                trader.generate_order_requests()
            }
            Self::GenerateOrderManual((_trader, _order)) => {
                // Transitions to one of:
                // Consume | Terminate
                unimplemented!()
            },
            Self::UpdateFromAccount((trader, account)) => {
                // Always transitions to: Consume
                trader.update(account)
            }
            Self::ExecuteCommand((trader, command)) => {
                // Transitions to one of:
                // Consume | GenerateOrder<Manual> | Terminate
                trader.execute_manual_command(command)
            }
            Self::Terminate(trader) => {
                Self::Terminate(trader)
            }
        }
    }
}

/// Builder to construct [`Engine`] instances.
#[derive(Default)]
pub struct EngineBuilder<Strategy, Portfolio> {
    pub feed: Option<EventFeed>,
    pub strategy: Option<Strategy>,
    pub execution_tx: Option<mpsc::UnboundedSender<ExecutionRequest>>,
    pub instruments: Option<HashMap<Exchange, Vec<Instrument>>>,
    pub phantom: PhantomData<Portfolio>,
}

impl<Strategy, Portfolio> EngineBuilder<Strategy, Portfolio>
where
    Strategy: OrderGenerator,
    Portfolio: MarketUpdater + AccountUpdater,
{
    fn new() -> Self {
        Self {
            feed: None,
            strategy: None,
            execution_tx: None,
            instruments: None,
            phantom: PhantomData::default()
        }
    }

    pub fn feed(self, value: EventFeed) -> Self {
        Self {
            feed: Some(value),
            ..self
        }
    }

    pub fn strategy(self, value: Strategy) -> Self {
        Self {
            strategy: Some(value),
            ..self
        }
    }

    pub fn execution_tx(self, value: mpsc::UnboundedSender<ExecutionRequest>) -> Self {
        Self {
            execution_tx: Some(value),
            ..self
        }
    }

    pub fn instruments(self, value: HashMap<Exchange, Vec<Instrument>>) -> Self {
        Self {
            instruments: Some(value),
            ..self
        }
    }

    pub fn build(self) -> Result<Engine<Strategy, Portfolio>, EngineError> {
        Ok(Engine::Initialise(Trader {
            feed: self.feed.ok_or(EngineError::BuilderIncomplete("feed"))?,
            strategy: self.strategy.ok_or(EngineError::BuilderIncomplete("strategy"))?,
            execution_tx: self.execution_tx.ok_or(EngineError::BuilderIncomplete("execution_tx"))?,
            state: Initialise {
                instruments: self.instruments.ok_or(EngineError::BuilderIncomplete("instruments"))?,
                phantom: self.phantom
            }
        }))
    }
}