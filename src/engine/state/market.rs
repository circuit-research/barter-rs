use barter_data::model::MarketEvent;
use super::{
    order::{GenerateOrder, Algorithmic},
};
use crate::engine::{Engine, Trader};
use crate::portfolio::{AccountUpdater, MarketUpdater};

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
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn update(self, _market: MarketEvent) -> Engine<Strategy, Portfolio> {
        // Todo:
        //  - Update Positions
        //  - Update Indicators
        Engine::GenerateOrder(Trader::from(self))
    }
}

/// a) UpdateFromMarket -> GenerateOrder<Algorithmic>
impl<Strategy, Portfolio> From<Trader<Strategy, UpdateFromMarket<Portfolio> >> for Trader<Strategy, GenerateOrder<Algorithmic>>
where
    Portfolio: MarketUpdater
{
    fn from(trader: Trader<Strategy, UpdateFromMarket<Portfolio> >) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: GenerateOrder { state: Algorithmic },
        }
    }
}