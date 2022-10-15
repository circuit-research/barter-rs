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

/// [`ExecuteCommand`] can transition to one of:
/// a) [`Consume`]
/// b) [`GenerateOrder<Manual>`](GenerateOrder)
/// c) [`Terminate`]
pub struct ExecuteCommand;

impl Trader<ExecuteCommand> {
    pub fn execute_manual_command(self, command: Command) -> Engine {
        match command {
            Command::FetchOpenPositions => {
                info!(kind = "Command", payload = "FetchOpenPositions", "received Event");
                // Todo: Send data to audit_tx
                Engine::Consume(Trader::from(self))
            }
            Command::ExitPosition => {
                info!(kind = "Command", payload = "ExitPosition", "received Event");
                // Todo: Add relevant metadata for the Position to exit
                Engine::GenerateOrderManual((Trader::from(self), ()))
            }
            Command::ExitAllPositions => {
                info!(kind = "Command", payload = "ExitAllPositions", "received Event");
                // Todo: Add relevant metadata for the Position to exit
                Engine::GenerateOrderManual((Trader::from(self), ()))
            }
            Command::Terminate => {
                info!(kind = "Command", payload = "Terminate", "received Event");
                // Todo: Do pre-termination tasks
                Engine::Terminate(Trader::from(self))
            }
        }
    }
}



/// a) Commander -> Consume
impl From<Trader<ExecuteCommand>> for Trader<Consume> {
    fn from(trader: Trader<ExecuteCommand>) -> Self {
        Self {
            state: Consume,
            feed: trader.feed,
        }
    }
}

/// b) ExecuteCommand -> GenerateOrder<Manual>
impl From<Trader<ExecuteCommand>> for Trader<GenerateOrder<Manual>> {
    fn from(trader: Trader<ExecuteCommand>) -> Self {
        Self {
            state: GenerateOrder { state: Manual },
            feed: trader.feed,
        }
    }
}

/// c) Commander -> Terminated
impl From<Trader<ExecuteCommand>> for Trader<Terminate> {
    fn from(trader: Trader<ExecuteCommand>) -> Self {
        Self {
            state: Terminate {
                reason: Ok("Command::Terminate")
            },
            feed: trader.feed,
        }
    }
}

