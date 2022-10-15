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

impl<Strategy, Execution> Trader<Strategy, Execution, ExecuteCommand> {
    pub fn execute_manual_command(self, command: Command) -> Engine<Strategy, Execution> {
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
impl<Strategy, Execution> From<Trader<Strategy, Execution, ExecuteCommand>> for Trader<Strategy, Execution, Consume> {
    fn from(trader: Trader<Strategy, Execution, ExecuteCommand>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution: trader.execution,
            state: Consume,
        }
    }
}

/// b) ExecuteCommand -> GenerateOrder<Manual>
impl<Strategy, Execution> From<Trader<Strategy, Execution, ExecuteCommand>> for Trader<Strategy, Execution, GenerateOrder<Manual>> {
    fn from(trader: Trader<Strategy, Execution, ExecuteCommand>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution: trader.execution,
            state: GenerateOrder { state: Manual },
        }
    }
}

/// c) Commander -> Terminated
impl<Strategy, Execution> From<Trader<Strategy, Execution, ExecuteCommand>> for Trader<Strategy, Execution, Terminate> {
    fn from(trader: Trader<Strategy, Execution, ExecuteCommand>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution: trader.execution,
            state: Terminate {
                reason: Ok("Command::Terminate")
            },
        }
    }
}

