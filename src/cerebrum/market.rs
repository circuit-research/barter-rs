use crate::cerebrum::Cerebrum;
use crate::cerebrum::order::Algorithmic;
use crate::cerebrum::OrderGenerator;
use barter_data::model::{DataKind, MarketEvent};

/// MarketUpdater can transition to:
///  a) OrderGenerator<Algorithmic>
pub struct MarketUpdater {
    pub market: MarketEvent
}

impl Cerebrum<MarketUpdater> {
    fn update(mut self) -> Cerebrum<OrderGenerator<Algorithmic>> {
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

/// a) MarketUpdater -> OrderGenerator<Algorithmic>
impl From<Cerebrum<MarketUpdater>> for Cerebrum<OrderGenerator<Algorithmic>> {
    fn from(cerebrum: Cerebrum<MarketUpdater>) -> Self {
        Self {
            feed: cerebrum.feed,
            event_tx: cerebrum.event_tx,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: OrderGenerator { state: Algorithmic }
        }
    }
}
