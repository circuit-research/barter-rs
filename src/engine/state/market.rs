use barter_data::model::MarketEvent;
use super::{
    order::{GenerateOrder, Algorithmic},
};
use crate::engine::{Engine, Trader};

/// [`UpdateFromMarket`] can only transition to:
/// a) [`GenerateOrder<Algorithmic>`](GenerateOrder)
pub struct UpdateFromMarket;

impl<Strategy, Execution> Trader<Strategy, Execution, UpdateFromMarket> {
    pub fn update(self, _market: MarketEvent) -> Engine<Strategy, Execution> {
        // Todo:
        //  - Update Positions
        //  - Update Indicators
        Engine::GenerateOrder(Trader::from(self))
    }
}

/// a) UpdateFromMarket -> GenerateOrder<Algorithmic>
impl<Strategy, Execution> From<Trader<Strategy, Execution, UpdateFromMarket>> for Trader<Strategy, Execution, GenerateOrder<Algorithmic>> {
    fn from(trader: Trader<Strategy, Execution, UpdateFromMarket>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution: trader.execution,
            state: GenerateOrder { state: Algorithmic },
        }
    }
}