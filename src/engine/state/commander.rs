use super::{
    order::{OrderGenerator, Manual},
    terminated::Terminated,
};
use crate::{
    event::Command,
    engine::{Engine, Trader}
};
use tracing::info;

/// [`Commander`] can transition to one of:
/// a) [`OrderGenerator<Manual>`](OrderGenerator)
/// b) [`Terminated`]
pub struct Commander;

impl Trader<Commander> {
    pub fn execute_manual_command(self, command: Command) -> Engine {
        match command {
            Command::Terminate => {
                info!(kind = "Command", payload = "Terminate", "received Event");
                // Todo: Do pre-termination tasks
                Engine::Terminated(Trader::from(self))
            }
            Command::FetchOpenPositions => {
                info!(kind = "Command", payload = "FetchOpenPositions", "received Event");
                // Todo: Send data to audit_tx
                Engine::Terminated(Trader::from(self))
            }
            Command::ExitPosition => {
                info!(kind = "Command", payload = "ExitPosition", "received Event");
                // Todo: Add relevant metadata for the Position to exit
                Engine::OrderGeneratorManual((Trader::from(self), ()))
            }
            Command::ExitAllPositions => {
                info!(kind = "Command", payload = "ExitAllPositions", "received Event");
                // Todo: Add relevant metadata for the Position to exit
                Engine::OrderGeneratorManual((Trader::from(self), ()))
            }
        }
    }
}

/// a) Commander -> OrderGenerator<Manual>
impl From<Trader<Commander>> for Trader<OrderGenerator<Manual>> {
    fn from(trader: Trader<Commander>) -> Self {
        Self {
            state: OrderGenerator { state: Manual },
            feed: trader.feed,
        }
    }
}

/// a) Commander -> Terminated
impl From<Trader<Commander>> for Trader<Terminated> {
    fn from(trader: Trader<Commander>) -> Self {
        Self {
            state: Terminated,
            feed: trader.feed,
        }
    }
}

