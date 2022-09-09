use crate::cerebrum::{Cerebrum, CerebrumState, End};
use crate::cerebrum::event::Command;
use crate::cerebrum::order::{Manual, OrderGenerator};

/// Commander can transition to:
///  a) End
///  b) OrderGenerator<Manual>
pub struct Commander {
    pub command: Command,
}

impl Cerebrum<Commander> {
    fn update(mut self) -> CerebrumState {
        // Action Command
        match self.state.command {
            Command::Terminate => {
                // Todo: Do pre-termination tasks
                CerebrumState::End(Cerebrum::from(self))
            }
            Command::FetchOpenPositions => {
                // Todo: Send data to event_tx
                CerebrumState::End(Cerebrum::from(self))
            }
            Command::ExitPosition => {
                // Todo: Add relevant metadata for the Position to exit
                CerebrumState::OrderGeneratorManual(Cerebrum::from((self, ())))
            }
            Command::ExitAllPositions => {
                // Todo: Add relevant metadata for the Position to exit
                CerebrumState::OrderGeneratorManual(Cerebrum::from((self, ())))
            }
        }
    }
}

/// a) Commander -> End
impl From<Cerebrum<Commander>> for Cerebrum<End> {
    fn from(cerebrum: Cerebrum<Commander>) -> Self {
        Self {
            feed: cerebrum.feed,
            event_tx: cerebrum.event_tx,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: End
        }
    }
}

/// b) Commander -> OrderGenerator<Manual>
impl From<(Cerebrum<Commander>, ())> for Cerebrum<OrderGenerator<Manual>> {
    fn from((cerebrum, meta): (Cerebrum<Commander>, ())) -> Self {
        Self {
            feed: cerebrum.feed,
            event_tx: cerebrum.event_tx,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: OrderGenerator { state: Manual { meta }}
        }
    }
}

