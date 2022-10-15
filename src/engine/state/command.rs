use super::{
    consume::Consume,
    order::{GenerateOrder, Manual},
    terminate::Terminate,
};
use crate::{
    event::Command,
    engine::{Engine, Trader},
    portfolio::{AccountUpdater, MarketUpdater},
    strategy::OrderGenerator,
};
use tracing::info;

/// [`ExecuteCommand`] can transition to one of:
/// a) [`Consume`]
/// b) [`GenerateOrder<Manual>`](GenerateOrder)
/// c) [`Terminate`]
pub struct ExecuteCommand<Portfolio> {
    pub portfolio: Portfolio,
}

impl<Strategy, Portfolio> Trader<Strategy, ExecuteCommand<Portfolio>>
where
    Strategy: OrderGenerator,
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn execute_manual_command(self, command: Command) -> Engine<Strategy, Portfolio> {
        info!(payload = ?command, "received Command");

        match command {
            Command::FetchOpenPositions => {
                // Todo: Fetch & send (where?)
                // Transition Engine state to Consume
                // Engine::Consume(Trader::from(self))
                unimplemented!()
            }
            Command::ExitPosition => {
                // Todo: Add relevant Order<RequestOpen> for Position to exit
                // Transition Engine state to GenerateOrder<Manual>
                // Engine::GenerateOrderManual((Trader::from(self), ()))
                unimplemented!()
            }
            Command::ExitAllPositions => {
                // Todo: Add relevant Vec<Order<RequestOpen>> for the all Positions to exit
                // Transition Engine state to GenerateOrder<Manual>
                // Engine::GenerateOrderManual((Trader::from(self), ()))
                unimplemented!()
            }
            Command::Terminate => {
                // Todo: Do pre-termination tasks
                // Transition Engine state to Terminate
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
impl<Strategy, Portfolio> From<Trader<Strategy, ExecuteCommand<Portfolio>>> for Trader<Strategy, GenerateOrder<Portfolio, Manual>> {
    fn from(trader: Trader<Strategy, ExecuteCommand<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: GenerateOrder {
                portfolio: trader.state.portfolio,
                kind: Manual
            },
        }
    }
}

/// c) Commander -> Terminated
impl<Strategy, Portfolio> From<Trader<Strategy, ExecuteCommand<Portfolio>>> for Trader<Strategy, Terminate<Portfolio>> {
    fn from(trader: Trader<Strategy, ExecuteCommand<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Terminate {
                portfolio: Some(trader.state.portfolio),
                reason: Ok("Command::Terminate")
            },
        }
    }
}
