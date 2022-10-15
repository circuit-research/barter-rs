use super::{
    consume::Consume,
    order::{GenerateOrder, Manual},
    terminate::Terminate,
};
use crate::{
    event::Command,
    engine::{Engine, Trader}
};
use tracing::info;
use crate::portfolio::{AccountUpdater, MarketUpdater};

/// [`ExecuteCommand`] can transition to one of:
/// a) [`Consume`]
/// b) [`GenerateOrder<Manual>`](GenerateOrder)
/// c) [`Terminate`]
pub struct ExecuteCommand<Portfolio> {
    pub portfolio: Portfolio,
}

impl<Strategy, Portfolio> Trader<Strategy, ExecuteCommand<Portfolio>>
where
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn execute_manual_command(self, command: Command) -> Engine<Strategy, Portfolio> {
        info!(payload = ?command, "received Command");

        match command {
            Command::FetchOpenPositions => {
                // Todo: Fetch & send (where?)
                Engine::Consume(Trader::from(self))
            }
            Command::ExitPosition => {
                // Todo: Add relevant metadata for the Position to exit
                Engine::GenerateOrderManual((Trader::from(self), ()))
            }
            Command::ExitAllPositions => {
                // Todo: Add relevant metadata for the Position to exit
                Engine::GenerateOrderManual((Trader::from(self), ()))
            }
            Command::Terminate => {
                // Todo: Do pre-termination tasks
                Engine::Terminate(Trader::from(self))
            }
        }
    }
}

/// a) Commander -> Consume
impl<Strategy, Portfolio> From<Trader<Strategy, ExecuteCommand<Portfolio>>> for Trader<Strategy, Consume<Portfolio>> {
    fn from(trader: Trader<Strategy, ExecuteCommand<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Consume {
                portfolio: trader.state.portfolio
            },
        }
    }
}

/// b) ExecuteCommand -> GenerateOrder<Manual>
impl<Strategy, Portfolio> From<Trader<Strategy, ExecuteCommand<Portfolio>>> for Trader<Strategy, GenerateOrder<Manual>> {
    fn from(trader: Trader<Strategy, ExecuteCommand<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: GenerateOrder { state: Manual },
        }
    }
}

/// c) Commander -> Terminated
impl<Strategy, Portfolio> From<Trader<Strategy, ExecuteCommand<Portfolio>>> for Trader<Strategy, Terminate> {
    fn from(trader: Trader<Strategy, ExecuteCommand<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Terminate {
                reason: Ok("Command::Terminate")
            },
        }
    }
}
