use crate::cerebrum::{Cerebrum, Engine};
use crate::cerebrum::consumer::Consumer;
use crate::cerebrum::event::AccountEvent;

/// AccountUpdater can transition to:
///  a) Consumer
pub struct AccountUpdater {
    pub account: AccountEvent,
}

impl Cerebrum<AccountUpdater> {
    pub fn update_from_account_event(mut self) -> Engine {
        // Update Positions, Statistics, Indicators
        match self.state.account {
            AccountEvent::OrderNew => {
                todo!()
            }
            AccountEvent::OrderCancelled => {
                todo!()
            }
            AccountEvent::Trade => {
                todo!()
            }
            AccountEvent::Balances => {
                todo!()
            }
        };

        Engine::Consumer(Cerebrum::from(self))
    }
}

/// a) AccountUpdater -> Consumer
impl From<Cerebrum<AccountUpdater>> for Cerebrum<Consumer> {
    fn from(cerebrum: Cerebrum<AccountUpdater>) -> Self {
        Self {
            state: Consumer,
            feed: cerebrum.feed,
            event_tx: cerebrum.event_tx,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
        }
    }
}