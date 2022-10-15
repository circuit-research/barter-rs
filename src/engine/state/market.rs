use tracing::info;
use barter_data::model::MarketEvent;
use super::{
    order::{GenerateOrder, Algorithmic},
};
use crate::engine::{Engine, Trader};
use crate::portfolio::{AccountUpdater, MarketUpdater};
use crate::strategy::OrderGenerator;

/// [`UpdateFromMarket`] can only transition to:
/// a) [`GenerateOrder<Algorithmic>`](GenerateOrder)
pub struct UpdateFromMarket<Portfolio>
where
    Portfolio: MarketUpdater
{
    pub portfolio: Portfolio,
}

impl<Strategy, Portfolio> Trader<Strategy, UpdateFromMarket<Portfolio>>
where
    Strategy: MarketUpdater + OrderGenerator,
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn update(mut self, market: MarketEvent) -> Engine<Strategy, Portfolio> {
        info!(
            exchange = ?market.exchange,
            instrument = %market.instrument,
            payload = ?market.kind,
            "received MarketEvent",
        );

        // Update Strategy
        self.strategy.update_from_market(&market);

        // Update Positions
        self.state.portfolio.update_from_market(&market);

        // Transition Engine state to GenerateOrder<Algorithmic>
        Engine::GenerateOrderAlgorithmic(Trader::from(self))
    }
}

/// a) UpdateFromMarket -> GenerateOrder<Algorithmic>
impl<Strategy, Portfolio> From<Trader<Strategy, UpdateFromMarket<Portfolio> >> for Trader<Strategy, GenerateOrder<Portfolio, Algorithmic>>
where
    Portfolio: MarketUpdater
{
    fn from(trader: Trader<Strategy, UpdateFromMarket<Portfolio> >) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: GenerateOrder {
                portfolio: trader.state.portfolio,
                kind: Algorithmic,
            },
        }
    }
}
