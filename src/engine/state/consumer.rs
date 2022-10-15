use super::{
    market::MarketUpdater,
    account::AccountUpdater,
    commander::Commander,
    terminated::Terminated,
};
use crate::{
    event::{Feed, Event, Command},
    engine::{
        Engine, Trader,
    }
};
use barter_data::model::MarketEvent;

/// [`Consumer`] can transition to one of:
/// a) [`MarketUpdater`]
/// b) [`AccountUpdater`]
/// c) [`Commander`]
/// d) [`Terminated`]
#[derive(Debug)]
pub struct Consumer;

impl Trader<Consumer> {
    pub fn next_event(mut self) -> Engine {
        // Consume next Event
        match self.feed.next() {
            Feed::Next(Event::Market(market)) => {
                Engine::MarketUpdater((Trader::from(self), market))
            }
            Feed::Next(Event::Account(account)) => {
                Engine::AccountUpdater((Trader::from(self), account))
            }
            Feed::Next(Event::Command(command)) => {
                Engine::Commander((Trader::from(self), command))
            }
            Feed::Finished => {
                Engine::Terminated(Trader::from(self))
            }
        }
    }
}

/// a) Consumer -> MarketUpdater
impl From<Trader<Consumer>> for Trader<MarketUpdater> {
    fn from(trader: Trader<Consumer>) -> Self {
        Self {
            state: MarketUpdater,
            feed: trader.feed,
        }
    }
}

/// b) Consumer -> AccountUpdater
impl From<Trader<Consumer>> for Trader<AccountUpdater> {
    fn from(trader: Trader<Consumer>) -> Self {
        Self {
            state: AccountUpdater,
            feed: trader.feed,
        }
    }
}

/// c) Consumer -> Commander
impl From<Trader<Consumer>> for Trader<Commander> {
    fn from(trader: Trader<Consumer>) -> Self {
        Self {
            state: Commander,
            feed: trader.feed,
        }
    }
}

/// d) Consumer -> Terminated
impl From<Trader<Consumer>> for Trader<Terminated> {
    fn from(trader: Trader<Consumer>) -> Self {
        Self {
            state: Terminated,
            feed: trader.feed,
        }
    }
}