use crate::cerebrum::Cerebrum;
use barter_data::model::{DataKind, MarketEvent};
use crate::cerebrum::OrderGenerator;

/// MarketUpdater can transition to:
///  a) OrderGenerator
pub struct MarketUpdater {
    pub market: MarketEvent
}

impl Cerebrum<MarketUpdater> {
    fn update(mut self) -> Cerebrum<OrderGenerator> {
        // Update Positions, Statistics, Indicators
        match self.state.market.kind {
            DataKind::Trade(_) => {
                todo!()
            }
            DataKind::Candle(_) => {
                todo!()
            }
        };

        Cerebrum::from(self)
    }
}

/// a) MarketUpdater -> OrderGenerator
impl From<Cerebrum<MarketUpdater>> for Cerebrum<OrderGenerator> {
    fn from(cerebrum: Cerebrum<MarketUpdater>) -> Self {
        Self {
            feed: cerebrum.feed,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: OrderGenerator
        }
    }
}
