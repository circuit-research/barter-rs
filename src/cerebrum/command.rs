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

impl<Strategy> Cerebrum<Commander, Strategy> {
    pub fn action_manual_command(mut self) -> Engine<Strategy> {
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
impl<Strategy> From<Cerebrum<Commander, Strategy>> for Cerebrum<Terminated, Strategy> {
    fn from(cerebrum: Cerebrum<Commander, Strategy>) -> Self {
        Self {
            state: Terminated,
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}

/// b) Commander -> OrderGenerator<Manual>
impl<Strategy> From<(Cerebrum<Commander, Strategy>, ())> for Cerebrum<OrderGenerator<Manual>, Strategy> {
    fn from((cerebrum, meta): (Cerebrum<Commander, Strategy>, ())) -> Self {
        Self {
            state: OrderGenerator { state: Manual { meta }},
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}

