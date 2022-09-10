use super::{Cerebrum, Engine, consume::Consumer, terminate::Terminated};

/// Initialiser can transition to one of:
///  a) Consumer
///  b) Terminated
pub struct Initialiser;

impl<Strategy> Cerebrum<Initialiser, Strategy> {
    pub fn init(mut self) -> Engine<Strategy> {
        // Todo: Hit ExchangeClient to get balances, orders, positions (may fail)
        // Todo: Add failure transition to Engine::Terminated if it's unrecoverable
        Engine::Consumer(Cerebrum::from(self))
    }
}

/// a) Initialiser -> Consumer
impl<Strategy> From<Cerebrum<Initialiser, Strategy>> for Cerebrum<Consumer, Strategy> {
    fn from(cerebrum: Cerebrum<Initialiser, Strategy>) -> Self {
        Self {
            state: Consumer,
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            audit_tx: cerebrum.audit_tx,
        }
    }
}

/// b) Initialiser -> Terminated
impl<Strategy> From<Cerebrum<Initialiser, Strategy>> for Cerebrum<Terminated, Strategy> {
    fn from(cerebrum: Cerebrum<Initialiser, Strategy>) -> Self {
        Self {
            state: Terminated,
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            audit_tx: cerebrum.audit_tx,
        }
    }
}