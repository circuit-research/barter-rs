use super::{
    consume::Consume,
};
use crate::engine::{Engine, Trader};
use barter_execution::model::{AccountEvent, AccountEventKind};
use tracing::info;

/// [`UpdateFromAccount`] can only transition to:
/// a) [`Consume`]
pub struct UpdateFromAccount;

impl Trader<UpdateFromAccount> {
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

        Engine::Consume(Trader::from(self))
    }
}

/// a) UpdateFromAccount -> Consume
impl From<Trader<UpdateFromAccount>> for Trader<Consume> {
    fn from(trader: Trader<UpdateFromAccount>) -> Self {
        Self {
            state: Consume,
            feed: trader.feed
        }
    }
}