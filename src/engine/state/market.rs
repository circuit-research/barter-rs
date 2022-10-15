use barter_data::model::MarketEvent;
use super::{
    order::{GenerateOrder, Algorithmic},
};
use crate::engine::{Engine, Trader};

/// [`UpdateFromMarket`] can only transition to:
/// a) [`GenerateOrder<Algorithmic>`](GenerateOrder)
pub struct UpdateFromMarket;

impl<Strategy> Trader<Strategy, UpdateFromMarket> {
    pub fn update(self, _market: MarketEvent) -> Engine<Strategy> {
        // Todo:
        //  - Update Positions
        //  - Update Indicators
        Engine::GenerateOrder(Trader::from(self))
    }
}

/// a) UpdateFromMarket -> GenerateOrder<Algorithmic>
impl<Strategy> From<Trader<Strategy, UpdateFromMarket>> for Trader<Strategy, GenerateOrder<Algorithmic>> {
    fn from(trader: Trader<Strategy, UpdateFromMarket>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: GenerateOrder { state: Algorithmic },
        }
    }
}