use crate::cerebrum::{Cerebrum, Engine, Terminated};
use crate::cerebrum::event::Command;
use crate::cerebrum::order::{Manual, OrderGenerator};

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
                Engine::Terminated(Cerebrum::from(self))
            }
            Command::FetchOpenPositions => {
                // Todo: Send data to event_tx
                Engine::Terminated(Cerebrum::from(self))
            }
            Command::ExitPosition => {
                // Todo: Add relevant metadata for the Position to exit
                Engine::OrderGeneratorManual(Cerebrum::from((self, ())))
            }
            Command::ExitAllPositions => {
                // Todo: Add relevant metadata for the Position to exit
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
            event_tx: cerebrum.event_tx,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
        }
    }
}

/// b) Commander -> OrderGenerator<Manual>
impl From<(Cerebrum<Commander>, ())> for Cerebrum<OrderGenerator<Manual>> {
    fn from((cerebrum, meta): (Cerebrum<Commander>, ())) -> Self {
        Self {
            state: OrderGenerator { state: Manual { meta }},
            feed: cerebrum.feed,
            event_tx: cerebrum.event_tx,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
        }
    }
}

