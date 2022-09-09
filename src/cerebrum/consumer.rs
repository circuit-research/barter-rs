use crate::cerebrum::{
    Cerebrum, CerebrumState, Commander
};
use barter_data::model::MarketEvent;
use tokio::sync::mpsc;
use crate::cerebrum::account::AccountUpdater;
use crate::cerebrum::event::{AccountEvent, Command, Event, EventFeed};
use crate::cerebrum::market::MarketUpdater;

/// Consumer can transition to one of:
///  a) MarketUpdater
///  b) AccountUpdater
///  c) Commander
pub struct Consumer;

impl Cerebrum<Consumer> {
    fn consume(mut self) -> CerebrumState {
        // Consume next Event
        match self.feed.next() {
            Event::Market(market) => {
                CerebrumState::MarketUpdater(Cerebrum::from((self, market)))
            }
            Event::Account(account) => {
                CerebrumState::AccountUpdater(Cerebrum::from((self, account)))
            }
            Event::Command(command) => {
                CerebrumState::Commander(Cerebrum::from((self, command)))
            }
        }
    }
}

/// a) Consumer -> MarketUpdater
impl From<(Cerebrum<Consumer>, MarketEvent)> for Cerebrum<MarketUpdater> {
    fn from((cerebrum, market): (Cerebrum<Consumer>, MarketEvent)) -> Self {
        Self {
            feed: cerebrum.feed,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: MarketUpdater { market }
        }
    }
}

/// b) Consumer -> AccountUpdater
impl From<(Cerebrum<Consumer>, AccountEvent)> for Cerebrum<AccountUpdater> {
    fn from((cerebrum, account): (Cerebrum<Consumer>, AccountEvent)) -> Self {
        Self {
            feed: cerebrum.feed,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: AccountUpdater { account }
        }
    }
}

/// c) Consumer -> Commander
impl From<(Cerebrum<Consumer>, Command)> for Cerebrum<Commander> {
    fn from((cerebrum, command): (Cerebrum<Consumer>, Command)) -> Self {
        Self {
            feed: cerebrum.feed,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: Commander { command }
        }
    }
}
