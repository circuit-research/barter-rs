use barter_data::model::MarketEvent;
use super::{
    order::{OrderGenerator, Algorithmic},
};
use crate::engine::{Engine, Trader};

/// [`MarketUpdater`] can only transition to:
/// a) [`OrderGenerator<Algorithmic>`](OrderGenerator)
pub struct MarketUpdater;

impl Trader<MarketUpdater> {
    pub fn update(self, _market: MarketEvent) -> Engine {
        // Todo:
        //  - Update Positions
        //  - Update Indicators
        Engine::OrderGenerator(Trader::from(self))
    }
}

/// a) MarketUpdater -> OrderGenerator<Algorithmic>
impl From<Trader<MarketUpdater>> for Trader<OrderGenerator<Algorithmic>> {
    fn from(trader: Trader<MarketUpdater>) -> Self {
        Self {
            state: OrderGenerator { state: Algorithmic },
            feed: trader.feed
        }
    }
}