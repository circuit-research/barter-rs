use super::{
    Cerebrum, Engine,
    event::{AccountEvent, Command, Event},
    market::MarketUpdater,
    account::AccountUpdater,
    command::Commander,
};
use barter_data::model::MarketEvent;

/// Consumer can transition to one of:
///  a) MarketUpdater
///  b) AccountUpdater
///  c) Commander
pub struct Consumer;

impl Cerebrum<Consumer> {
    pub fn next_event(mut self) -> Engine {
        // Consume next Event
        match self.feed.next() {
            Event::Market(market) => {
                Engine::MarketUpdater(Cerebrum::from((self, market)))
            }
            Event::Account(account) => {
                Engine::AccountUpdater(Cerebrum::from((self, account)))
            }
            Event::Command(command) => {
                Engine::Commander(Cerebrum::from((self, command)))
            }
        }
    }
}

/// a) Consumer -> MarketUpdater
impl From<(Cerebrum<Consumer>, MarketEvent)> for Cerebrum<MarketUpdater> {
    fn from((cerebrum, market): (Cerebrum<Consumer>, MarketEvent)) -> Self {
        Self {
            state: MarketUpdater { market },
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}

/// b) Consumer -> AccountUpdater
impl From<(Cerebrum<Consumer>, AccountEvent)> for Cerebrum<AccountUpdater> {
    fn from((cerebrum, account): (Cerebrum<Consumer>, AccountEvent)) -> Self {
        Self {
            state: AccountUpdater { account },
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}

/// c) Consumer -> Commander
impl From<(Cerebrum<Consumer>, Command)> for Cerebrum<Commander> {
    fn from((cerebrum, command): (Cerebrum<Consumer>, Command)) -> Self {
        Self {
            state: Commander { command },
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}
