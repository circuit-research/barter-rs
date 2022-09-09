use crate::cerebrum::Cerebrum;
use crate::cerebrum::consumer::Consumer;
use crate::cerebrum::event::AccountEvent;

/// AccountUpdater can transition to:
///  a) Consumer
pub struct AccountUpdater {
    pub account: AccountEvent,
}

impl Cerebrum<AccountUpdater> {
    fn update(mut self) -> Cerebrum<Consumer> {
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

        Cerebrum::from(self)
    }
}

/// a) AccountUpdater -> Consumer
impl From<Cerebrum<AccountUpdater>> for Cerebrum<Consumer> {
    fn from(cerebrum: Cerebrum<AccountUpdater>) -> Self {
        Self {
            feed: cerebrum.feed,
            event_tx: cerebrum.event_tx,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: Consumer
        }
    }
}