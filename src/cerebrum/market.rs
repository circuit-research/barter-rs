use super::{
    Cerebrum, Engine, OrderGenerator,
    order::Algorithmic
};
use barter_data::model::{DataKind, MarketEvent};


pub trait IndicatorUpdater {
    fn update(&mut self, market: MarketEvent);
}

/// MarketUpdater can transition to:
///  a) OrderGenerator<Algorithmic>
pub struct MarketUpdater {
    pub market: MarketEvent
}

impl<Strategy> Cerebrum<MarketUpdater, Strategy>
where
    Strategy: IndicatorUpdater,
{
    pub fn update_from_market_event(mut self) -> Engine<Strategy> {
        Engine::OrderGeneratorAlgorithmic(Cerebrum::from(self))
    }
}

/// a) MarketUpdater -> OrderGenerator<Algorithmic>
impl<Strategy> From<Cerebrum<MarketUpdater, Strategy>> for Cerebrum<OrderGenerator<Algorithmic>, Strategy>
where
    Strategy: IndicatorUpdater
{
    fn from(mut cerebrum: Cerebrum<MarketUpdater, Strategy>) -> Self {
        // Destructure to access owned MarketUpdater State
        let Cerebrum { state, .. } = cerebrum;

        // Update Indicators
        cerebrum.strategy.update(state.market);

        Self {
            state: OrderGenerator { state: Algorithmic },
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}
