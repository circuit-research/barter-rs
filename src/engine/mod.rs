use state::{
    consumer::Consumer,
    market::MarketUpdater,
    order::{OrderGenerator, Algorithmic, Manual},
    account::AccountUpdater,
    commander::Commander,
    terminated::Terminated,
};
use crate::event::{Command, EventFeed};
use barter_data::model::MarketEvent;
use barter_execution::model::AccountEvent;

pub mod state;

pub struct Components {
    feed: EventFeed,
}

pub enum Engine {
    Consumer(Trader<Consumer>),
    MarketUpdater((Trader<MarketUpdater>, MarketEvent)),
    OrderGenerator(Trader<OrderGenerator<Algorithmic>>),
    OrderGeneratorManual((Trader<OrderGenerator<Manual>>, ())),
    AccountUpdater((Trader<AccountUpdater>, AccountEvent)),
    Commander((Trader<Commander>, Command)),
    Terminated(Trader<Terminated>)
}

pub struct Trader<State> {
    pub state: State,
    pub feed: EventFeed,
}

impl Engine {
    pub fn new(components: Components) -> Self {
        Self::Consumer(Trader {
            state: Consumer,
            feed: components.feed,
        })
    }

    pub fn run(mut self) {
        'trading: loop {
            // Transition to the next trading state
            self = self.next();

            if let Self::Terminated(_) = self {
                // Todo: Print trading session results & persist
                break 'trading
            }
        }
    }

    pub fn next(mut self) -> Self {
        match self {
            Self::Consumer(trader) => {
                trader.next_event()
            },
            Self::MarketUpdater((trader, market)) => {
                trader.update(market)
            },
            Self::OrderGenerator(trader) => {
                todo!()
            }
            Self::OrderGeneratorManual((trader, meta)) => {
                todo!()
            },
            Self::AccountUpdater((trader, account)) => {
                trader.update(account)
            }
            Self::Commander((trader, command)) => {
                trader.execute_manual_command(command)
            }
            Self::Terminated(trader) => {
                Self::Terminated(trader)
            }
        }
    }
}