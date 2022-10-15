use super::{
    consumer::Consumer,
};
use crate::engine::{Engine, Trader};
use barter_execution::model::{AccountEvent, AccountEventKind};
use tracing::info;

/// [`AccountUpdater`] can only transition to:
/// a) [`Consumer`]
pub struct AccountUpdater;

impl Trader<AccountUpdater> {
    pub fn update(self, account: AccountEvent) -> Engine {
        match account.kind {
            AccountEventKind::OrdersOpen(open) => {
                info!(kind = "Account", exchange = ?account.exchange, payload = ?open, "received Event");
            }
            AccountEventKind::OrdersNew(new) => {
                info!(kind = "Account", exchange = ?account.exchange, payload = ?new, "received Event");
            }
            AccountEventKind::OrdersCancelled(cancelled) => {
                info!(kind = "Account", exchange = ?account.exchange, payload = ?cancelled, "received Event");
            }
            AccountEventKind::Balance(balance) => {
                info!(kind = "Account", exchange = ?account.exchange, payload = ?balance, "received Event");
            }
            AccountEventKind::Balances(balances) => {
                info!(kind = "Account", exchange = ?account.exchange, payload = ?balances, "received Event");
            }
            AccountEventKind::Trade(trade) => {
                info!(kind = "Account", exchange = ?account.exchange, payload = ?trade, "received Event");
            }
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