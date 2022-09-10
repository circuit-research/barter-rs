use super::{
    Cerebrum, Engine, OrderGenerator,
    order::Algorithmic
};
use barter_data::model::{DataKind, MarketEvent};


pub trait IndicatorUpdater {
    fn update_indicators(&mut self, market: &MarketEvent);
}

/// MarketUpdater can transition to:
///  a) OrderGenerator<Algorithmic>
// pub struct MarketUpdater {
//     pub market: MarketEvent
// }
pub struct MarketUpdater;

impl<Strategy> Cerebrum<MarketUpdater, Strategy>
where
    Strategy: IndicatorUpdater,
{
    pub fn update_from_market(mut self, market: MarketEvent) -> Engine<Strategy> {
        // Update Positions
        self.accounts.update_positions(&market);

        // Update Indicators
        self.strategy.update_indicators(&market);

        Engine::OrderGeneratorAlgorithmic(Cerebrum::from(self))
    }
}

/// a) MarketUpdater -> OrderGenerator<Algorithmic>
impl<Strategy> From<Cerebrum<MarketUpdater, Strategy>> for Cerebrum<OrderGenerator<Algorithmic>, Strategy> {
    fn from(cerebrum: Cerebrum<MarketUpdater, Strategy>) -> Self {
        Self {
            state: OrderGenerator { state: Algorithmic },
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            audit_tx: cerebrum.audit_tx,
        }
    }
}
