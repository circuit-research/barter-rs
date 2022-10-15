use super::{
    market::UpdateFromMarket,
    account::UpdateFromAccount,
    command::ExecuteCommand,
    terminate::Terminate,
};
use crate::{
    event::{Feed, Event},
    engine::{
        Engine, Trader,
    }
};

/// [`Consume`] can transition to one of:
/// a) [`UpdateFromMarket`]
/// b) [`UpdateFromAccount`]
/// c) [`ExecuteCommand`]
/// d) [`Terminate`]
#[derive(Debug)]
pub struct Consume;

impl Trader<Consume> {
    pub fn next_event(mut self) -> Engine {
        // Consume next Event
        match self.feed.next() {
            Feed::Next(Event::Market(market)) => {
                Engine::UpdateFromMarket((Trader::from(self), market))
            }
            Feed::Next(Event::Account(account)) => {
                Engine::UpdateFromAccount((Trader::from(self), account))
            }
            Feed::Next(Event::Command(command)) => {
                Engine::ExecuteCommand((Trader::from(self), command))
            }
            Feed::Finished => {
                Engine::Terminate(Trader::from(self))
            }
        }
    }
}

/// a) Consume -> UpdateFromMarket
impl From<Trader<Consume>> for Trader<UpdateFromMarket> {
    fn from(trader: Trader<Consume>) -> Self {
        Self {
            state: UpdateFromMarket,
            feed: trader.feed,
        }
    }
}

/// b) Consume -> UpdateFromAccount
impl From<Trader<Consume>> for Trader<UpdateFromAccount> {
    fn from(trader: Trader<Consume>) -> Self {
        Self {
            state: UpdateFromAccount,
            feed: trader.feed,
        }
    }
}

/// c) Consume -> ExecuteCommand
impl From<Trader<Consume>> for Trader<ExecuteCommand> {
    fn from(trader: Trader<Consume>) -> Self {
        Self {
            state: ExecuteCommand,
            feed: trader.feed,
        }
    }
}

/// d) Consume -> Terminate
impl From<Trader<Consume>> for Trader<Terminate> {
    fn from(trader: Trader<Consume>) -> Self {
        todo!()
        // Self {
        //     state: Terminated {
        //         reason: ()
        //     },
        //     feed: trader.feed,
        // }
    }
}