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

impl<Strategy, Execution> Trader<Strategy, Execution, Consume> {
    pub fn next_event(mut self) -> Engine<Strategy, Execution> {
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
impl<Strategy, Execution> From<Trader<Strategy, Execution, Consume>> for Trader<Strategy, Execution, UpdateFromMarket> {
    fn from(trader: Trader<Strategy, Execution, Consume>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution: trader.execution,
            state: UpdateFromMarket,
        }
    }
}

/// b) Consume -> UpdateFromAccount
impl<Strategy, Execution> From<Trader<Strategy, Execution, Consume>> for Trader<Strategy, Execution, UpdateFromAccount> {
    fn from(trader: Trader<Strategy, Execution, Consume>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution: trader.execution,
            state: UpdateFromAccount,
        }
    }
}

/// c) Consume -> ExecuteCommand
impl<Strategy, Execution> From<Trader<Strategy, Execution, Consume>> for Trader<Strategy, Execution, ExecuteCommand> {
    fn from(trader: Trader<Strategy, Execution, Consume>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution: trader.execution,
            state: ExecuteCommand,
        }
    }
}

/// d) Consume -> Terminate
impl<Strategy, Execution> From<Trader<Strategy, Execution, Consume>> for Trader<Strategy, Execution, Terminate> {
    fn from(trader: Trader<Strategy, Execution, Consume>) -> Self {
        todo!()
        // Self {
        //     state: Terminated {
        //         reason: ()
        //     },
        //     feed: trader.feed,
        // }
    }
}