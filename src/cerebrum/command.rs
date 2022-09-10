use crate::cerebrum::{Cerebrum, Engine};
use crate::cerebrum::event::Command;
use crate::cerebrum::order::{Manual, OrderGenerator};
use crate::cerebrum::terminate::Terminated;

/// Commander can transition to:
///  a) End
///  b) OrderGenerator<Manual>
pub struct Commander {
    pub command: Command,
}

impl Cerebrum<Commander> {
    pub fn action_manual_command(mut self) -> Engine {
        // Action Command
        match self.state.command {
            Command::Terminate => {
                // Todo: Do pre-termination tasks
                println!("Received Command::Terminate");
                Engine::Terminated(Cerebrum::from(self))
            }
            Command::FetchOpenPositions => {
                // Todo: Send data to event_tx
                println!("Received Command::FetchOpenPositions");
                Engine::Terminated(Cerebrum::from(self))
            }
            Command::ExitPosition => {
                // Todo: Add relevant metadata for the Position to exit
                println!("Received Command::ExitPosition");
                Engine::OrderGeneratorManual(Cerebrum::from((self, ())))
            }
            Command::ExitAllPositions => {
                // Todo: Add relevant metadata for the Position to exit
                println!("Received Command::ExitAllPositions");
                Engine::OrderGeneratorManual(Cerebrum::from((self, ())))
            }
        }
    }
}

/// a) Commander -> End
impl From<Cerebrum<Commander>> for Cerebrum<Terminated> {
    fn from(cerebrum: Cerebrum<Commander>) -> Self {
        Self {
            state: Terminated,
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}

/// b) Commander -> OrderGenerator<Manual>
impl From<(Cerebrum<Commander>, ())> for Cerebrum<OrderGenerator<Manual>> {
    fn from((cerebrum, meta): (Cerebrum<Commander>, ())) -> Self {
        Self {
            state: OrderGenerator { state: Manual { meta }},
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}

