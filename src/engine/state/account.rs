use barter_execution::model::{AccountEvent, AccountEventKind};
use crate::engine::{Engine, Trader};
use crate::engine::state::consumer::Consumer;

/// [`AccountUpdater`] can only transition to:
/// a) [`Consumer`]
pub struct AccountUpdater;

impl Trader<AccountUpdater> {
    pub fn update(self, account: AccountEvent) -> Engine {
        // Todo: Update accounts
        match account.kind {
            AccountEventKind::OrdersOpen(_) => {}
            AccountEventKind::OrdersNew(_) => {}
            AccountEventKind::OrdersCancelled(_) => {}
            AccountEventKind::Balance(_) => {}
            AccountEventKind::Trade(_) => {}
            AccountEventKind::Balances(_) => {}
        }

        Engine::Consumer(Trader::from(self))
    }
}

/// a) AccountUpdater -> Consumer
impl From<Trader<AccountUpdater>> for Trader<Consumer> {
    fn from(trader: Trader<AccountUpdater>) -> Self {
        Self {
            state: Consumer,
            feed: trader.feed
        }
    }
}