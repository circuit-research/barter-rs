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
    },
    portfolio::{AccountUpdater, MarketUpdater}
};
use crate::strategy::OrderGenerator;

/// [`Consume`] can transition to one of:
/// a) [`UpdateFromMarket`]
/// b) [`UpdateFromAccount`]
/// c) [`ExecuteCommand`]
/// d) [`Terminate`]
#[derive(Debug)]
pub struct Consume<Portfolio> {
    pub portfolio: Portfolio
}

impl<Strategy, Portfolio> Trader<Strategy, Consume<Portfolio>>
where
    Strategy: OrderGenerator,
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn next_event(mut self) -> Engine<Strategy, Portfolio> {
        // Consume next Event & transition to the required state
        match self.feed.next() {
            Feed::Next(Event::Market(market)) => {
                // Transition Engine state to UpdateFromMarket
                Engine::UpdateFromMarket((Trader::from(self), market))
            }
            Feed::Next(Event::Account(account)) => {
                // Transition Engine state to UpdateFromAccount
                Engine::UpdateFromAccount((Trader::from(self), account))
            }
            Feed::Next(Event::Command(command)) => {
                // Transition Engine state to ExecuteCommand
                Engine::ExecuteCommand((Trader::from(self), command))
            }
            Feed::Finished => {
                // Transition Engine state to Terminate
                Engine::Terminate(Trader::from(self))
            }
        }
    }
}

/// a) Consume -> UpdateFromMarket
impl<Strategy, Portfolio> From<Trader<Strategy, Consume<Portfolio>>> for Trader<Strategy, UpdateFromMarket<Portfolio>>
where
    Portfolio: MarketUpdater,
{
    fn from(trader: Trader<Strategy, Consume<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: UpdateFromMarket {
                portfolio: trader.state.portfolio
            },
        }
    }
}

/// b) Consume -> UpdateFromAccount
impl<Strategy, Portfolio> From<Trader<Strategy, Consume<Portfolio>>> for Trader<Strategy, UpdateFromAccount<Portfolio>>
where
    Portfolio: AccountUpdater,
{
    fn from(trader: Trader<Strategy, Consume<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: UpdateFromAccount {
                portfolio: trader.state.portfolio,
            },
        }
    }
}

/// c) Consume -> ExecuteCommand
impl<Strategy, Portfolio> From<Trader<Strategy, Consume<Portfolio>>> for Trader<Strategy, ExecuteCommand<Portfolio>> {
    fn from(trader: Trader<Strategy, Consume<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: ExecuteCommand {
                portfolio: trader.state.portfolio,
            },
        }
    }
}

/// d) Consume -> Terminate
impl<Strategy, Portfolio> From<Trader<Strategy, Consume<Portfolio>>> for Trader<Strategy, Terminate> {
    fn from(trader: Trader<Strategy, Consume<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Terminate {
                reason: Ok("EventFeed Feed::Finished")
            }
        }
    }
}