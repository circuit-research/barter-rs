use state::{
    Initialise,
    consume::Consume,
    market::UpdateFromMarket,
    order::{GenerateOrder, Algorithmic, Manual},
    account::UpdateFromAccount,
    command::ExecuteCommand,
    terminate::Terminate,
};
use crate::event::{Command, EventFeed};
use barter_data::model::MarketEvent;
use barter_execution::model::AccountEvent;

pub mod state;
pub mod error;

// Todo:
//  - Should AccountEvent contain an exchange_timestamp?
//  - Trader should contain ExecutionManager generic
//   '--> simple case would be an ExecutionClient, complex would be mpsc::Sender<EXecutionReq>

pub struct Components {
    feed: EventFeed,
}

pub enum Engine {
    Initialise(Trader<Initialise>),
    Consume(Trader<Consume>),
    UpdateFromMarket((Trader<UpdateFromMarket>, MarketEvent)),
    GenerateOrder(Trader<GenerateOrder<Algorithmic>>),
    GenerateOrderManual((Trader<GenerateOrder<Manual>>, ())),
    UpdateFromAccount((Trader<UpdateFromAccount>, AccountEvent)),
    ExecuteCommand((Trader<ExecuteCommand>, Command)),
    Terminate(Trader<Terminate>)
}

pub struct Trader<State> {
    pub state: State,
    pub feed: EventFeed,

}

impl Engine {
    pub fn new(components: Components) -> Self {
        Self::Initialise(Trader {
            state: Initialise,
            feed: components.feed,
        })
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
                todo!()
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