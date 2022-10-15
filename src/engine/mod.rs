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

pub enum Engine {
    Consumer(Trader<Consumer>),
    MarketUpdater((Trader<MarketUpdater>, MarketEvent)),
    OrderGenerator(Trader<OrderGenerator<Algorithmic>>),
    OrderGeneratorManual(Trader<OrderGenerator<Manual>>),
    AccountUpdater((Trader<AccountUpdater>, AccountEvent)),
    Commander((Trader<Commander>, Command)),
    Terminated(Trader<Terminated>)
}

pub struct Trader<State> {
    pub state: State,
    pub feed: EventFeed,
}