use std::collections::HashMap;
use barter_integration::model::{Market, Side};
use uuid::Uuid;
use super::{
    Cerebrum, consume::Consumer,
    Engine,
    event::AccountEvent,
};

/// AccountUpdater can transition to:
///  a) Consumer
pub struct AccountUpdater {
    pub account: AccountEvent,
}

impl<Strategy> Cerebrum<AccountUpdater, Strategy> {
    pub fn update_from_account_event(mut self) -> Engine<Strategy> {
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
impl<Strategy> From<Cerebrum<AccountUpdater, Strategy>> for Cerebrum<Consumer, Strategy> {
    fn from(cerebrum: Cerebrum<AccountUpdater, Strategy>) -> Self {
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

pub struct Accounts {
    pub balances: HashMap<Market, Balance>,
    pub orders: Orders,
}

pub struct Balance {
    pub total: f64,
    pub available: f64,
}

pub struct Orders {
    pub in_flight: HashMap<ClientOrderId, ()>,
    pub open: HashMap<ClientOrderId, ()>,
}

pub struct ClientOrderId(pub Uuid);

impl Accounts {

}