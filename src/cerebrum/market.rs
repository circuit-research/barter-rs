use super::{
    Cerebrum, Engine, OrderGenerator,
    order::Algorithmic
};
use barter_data::model::{DataKind, MarketEvent};

/// MarketUpdater can transition to:
///  a) OrderGenerator<Algorithmic>
pub struct MarketUpdater {
    pub market: MarketEvent
}

impl<Strategy> Cerebrum<MarketUpdater, Strategy> {
    pub fn update_from_market_event(mut self) -> Engine<Strategy> {
        // Update Positions, Statistics, Indicators
        match &self.state.market.kind {
            DataKind::Trade(trade) => {
                println!("Update from market: {trade:?}");
            }
            DataKind::Candle(candle) => {
                println!("Update from market: {candle:?}");
            }
        };

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
            event_tx: cerebrum.event_tx,
        }
    }
}
