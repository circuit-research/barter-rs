use barter_data::model::MarketEvent;
use super::{
    order::{GenerateOrder, Algorithmic},
};
use crate::engine::{Engine, Trader};

/// [`UpdateFromMarket`] can only transition to:
/// a) [`GenerateOrder<Algorithmic>`](GenerateOrder)
pub struct UpdateFromMarket;

impl Trader<UpdateFromMarket> {
    pub fn update(self, _market: MarketEvent) -> Engine {
        // Todo:
        //  - Update Positions
        //  - Update Indicators
        Engine::GenerateOrder(Trader::from(self))
    }
}

/// a) UpdateFromMarket -> GenerateOrder<Algorithmic>
impl From<Trader<UpdateFromMarket>> for Trader<GenerateOrder<Algorithmic>> {
    fn from(trader: Trader<UpdateFromMarket>) -> Self {
        Self {
            state: GenerateOrder { state: Algorithmic },
            feed: trader.feed
        }
    }
}