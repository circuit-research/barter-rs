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
    pub fn update(mut self, account: AccountEvent) -> Engine<Strategy, Portfolio> {
        info!(exchange = ?account.exchange, payload = ?account.kind, "received AccountEvent");

        // Update Portfolio
        self.state.portfolio.update_from_account(&account);

        // Transition Engine state to Consume
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