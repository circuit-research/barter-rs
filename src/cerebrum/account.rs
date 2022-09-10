use super::{
    Engine, Cerebrum,
    event::AccountEvent,
    consume::Consumer,
};

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
                // Todo:
                println!("update_from_account: OrderNew");
            }
            AccountEvent::OrderCancelled => {
                // Todo:
                println!("update_from_account: OrderCancelled");
            }
            AccountEvent::Trade => {
                // Todo:
                println!("update_from_account: Trade");
            }
            AccountEvent::Balances => {
                // Todo:
                println!("update_from_account: Balances");
            }
            AccountEvent::ConnectionStatus(status) => {
                // Todo:
                println!("update_from_account: {status:?}");
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
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}