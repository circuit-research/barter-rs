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

impl<Strategy> Trader<Strategy, Consume> {
    pub fn next_event(mut self) -> Engine<Strategy> {
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
impl<Strategy> From<Trader<Strategy, Consume>> for Trader<Strategy, UpdateFromMarket> {
    fn from(trader: Trader<Strategy, Consume>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: UpdateFromMarket,
        }
    }
}

/// b) Consume -> UpdateFromAccount
impl<Strategy> From<Trader<Strategy, Consume>> for Trader<Strategy, UpdateFromAccount> {
    fn from(trader: Trader<Strategy, Consume>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: UpdateFromAccount,
        }
    }
}

/// c) Consume -> ExecuteCommand
impl<Strategy> From<Trader<Strategy, Consume>> for Trader<Strategy, ExecuteCommand> {
    fn from(trader: Trader<Strategy, Consume>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: ExecuteCommand,
        }
    }
}

/// d) Consume -> Terminate
impl<Strategy> From<Trader<Strategy, Consume>> for Trader<Strategy, Terminate> {
    fn from(trader: Trader<Strategy, Consume>) -> Self {
        todo!()
        // Self {
        //     state: Terminated {
        //         reason: ()
        //     },
        //     feed: trader.feed,
        // }
    }
}