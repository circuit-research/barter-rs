use super::{
    Cerebrum, Engine, OrderGenerator,
    order::Algorithmic,
    strategy::IndicatorUpdater,
};
use barter_data::model::MarketEvent;
use tracing::info;

/// MarketUpdater can transition to:
///  a) OrderGenerator<Algorithmic>
pub struct MarketUpdater;

impl<Strategy> Cerebrum<MarketUpdater, Strategy>
where
    Strategy: IndicatorUpdater,
{
    pub fn update(mut self, market: MarketEvent) -> Engine<Strategy> {
        info!(kind = "Market", exchange = ?market.exchange, instrument = %market.instrument, payload = ?market, "received Event");

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
