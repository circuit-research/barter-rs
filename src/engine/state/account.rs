use super::{
    consume::Consume,
};
use crate::engine::{Engine, Trader};
use barter_execution::model::{AccountEvent, AccountEventKind};
use tracing::info;
use crate::portfolio::{AccountUpdater, MarketUpdater};

/// [`UpdateFromAccount`] can only transition to:
/// a) [`Consume`]
pub struct UpdateFromAccount<Portfolio>
where
    Portfolio: AccountUpdater,
{
    pub portfolio: Portfolio,
}

impl<Strategy, Portfolio> Trader<Strategy, UpdateFromAccount<Portfolio>>
where
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn update(self, account: AccountEvent) -> Engine<Strategy, Portfolio> {
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
impl<Strategy, Portfolio> From<Trader<Strategy, UpdateFromAccount<Portfolio>>> for Trader<Strategy, Consume<Portfolio>>
where
    Portfolio: AccountUpdater
{
    fn from(trader: Trader<Strategy, UpdateFromAccount<Portfolio>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Consume {
                portfolio: trader.state.portfolio
            },
        }
    }
}