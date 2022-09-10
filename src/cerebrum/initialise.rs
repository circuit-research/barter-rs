use super::{Cerebrum, Engine, consume::Consumer, terminate::Terminated};

/// Initialiser can transition to one of:
///  a) Consumer
///  b) Terminated
pub struct Initialiser;

impl Cerebrum<Initialiser> {
    pub fn init(mut self) -> Engine {
        // Todo: Hit ExchangeClient to get balances, orders, positions (may fail)
        // Todo: Add failure transition to Engine::Terminated if it's unrecoverable
        Engine::Consumer(Cerebrum::from(self))
    }
}

/// a) Initialiser -> Consumer
impl From<Cerebrum<Initialiser>> for Cerebrum<Consumer> {
    fn from(cerebrum: Cerebrum<Initialiser>) -> Self {
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

/// b) Initialiser -> Terminated
impl From<Cerebrum<Initialiser>> for Cerebrum<Terminated> {
    fn from(cerebrum: Cerebrum<Initialiser>) -> Self {
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