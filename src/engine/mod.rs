use self::{
    error::EngineError,
    state::{
        Initialise,
        consume::Consume,
        market::UpdateFromMarket,
        order::{GenerateOrder, Algorithmic, Manual},
        account::UpdateFromAccount,
        command::ExecuteCommand,
        terminate::Terminate,
    }
};
use crate::{
    event::{Command, EventFeed}
};
use barter_data::model::MarketEvent;
use barter_execution::model::AccountEvent;

pub mod state;
pub mod error;

// Todo:
//  - Should AccountEvent contain an exchange_timestamp?
//  - Trader should contain ExecutionManager generic
//   '--> simple case would be an ExecutionClient, complex would be mpsc::Sender<ExecutionRequest>

pub struct Components<Strategy, Execution> {
    pub feed: EventFeed,
    pub strategy: Strategy,
    pub execution: Execution,
}

pub enum Engine<Strategy, Execution> {
    Initialise(Trader<Strategy, Execution, Initialise>),
    Consume(Trader<Strategy, Execution, Consume>),
    UpdateFromMarket((Trader<Strategy, Execution, UpdateFromMarket>, MarketEvent)),
    GenerateOrder(Trader<Strategy, Execution, GenerateOrder<Algorithmic>>),
    GenerateOrderManual((Trader<Strategy, Execution, GenerateOrder<Manual>>, ())),
    UpdateFromAccount((Trader<Strategy, Execution, UpdateFromAccount>, AccountEvent)),
    ExecuteCommand((Trader<Strategy, Execution, ExecuteCommand>, Command)),
    Terminate(Trader<Strategy, Execution, Terminate>)
}

pub struct Trader<Strategy, Execution, State> {
    pub feed: EventFeed,
    pub strategy: Strategy,
    pub execution: Execution,
    pub state: State,
}

impl<Strategy, Execution> Engine<Strategy, Execution> {
    /// Builder to construct [`Engine`] instances.
    pub fn builder() -> EngineBuilder<Strategy, Execution> {
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

    pub fn next(mut self) -> Self {
        match self {
            Self::Initialise(trader) => {
                trader.init()
            }
            Self::Consume(trader) => {
                trader.next_event()
            },
            Self::UpdateFromMarket((trader, market)) => {
                trader.update(market)
            },
            Self::GenerateOrder(trader) => {
                todo!()
            }
            Self::GenerateOrderManual((trader, meta)) => {
                todo!()
            },
            Self::UpdateFromAccount((trader, account)) => {
                trader.update(account)
            }
            Self::ExecuteCommand((trader, command)) => {
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
pub struct EngineBuilder<Strategy, Execution> {
    pub feed: Option<EventFeed>,
    pub strategy: Option<Strategy>,
    pub execution: Option<Execution>,
}

impl<Strategy, Execution> EngineBuilder<Strategy, Execution> {
    fn new() -> Self {
        Self {
            feed: None,
            strategy: None,
            execution: None,
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

    pub fn execution(self, value: Execution) -> Self {
        Self {
            execution: Some(value),
            ..self
        }
    }

    pub fn build(self) -> Result<Engine<Strategy, Execution>, EngineError> {
        Ok(Engine::Initialise(Trader {
            feed: self.feed.ok_or(EngineError::BuilderIncomplete("feed"))?,
            strategy: self.strategy.ok_or(EngineError::BuilderIncomplete("strategy"))?,
            execution: self.execution.ok_or(EngineError::BuilderIncomplete("execution"))?,
            state: Initialise
        }))
    }
}