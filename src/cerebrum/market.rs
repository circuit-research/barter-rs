use crate::cerebrum::{Cerebrum, Engine};
use crate::cerebrum::order::Algorithmic;
use crate::cerebrum::OrderGenerator;
use barter_data::model::{DataKind, MarketEvent};

/// MarketUpdater can transition to:
///  a) OrderGenerator<Algorithmic>
pub struct MarketUpdater {
    pub market: MarketEvent
}

impl Cerebrum<MarketUpdater> {
    pub fn update_from_market_event(mut self) -> Engine {
        // Update Positions, Statistics, Indicators
        match self.state.market.kind {
            DataKind::Trade(_) => {
                todo!()
            }
            DataKind::Candle(_) => {
                todo!()
            }
        };

        Engine::OrderGeneratorAlgorithmic(Cerebrum::from(self))
    }
}

/// a) MarketUpdater -> OrderGenerator<Algorithmic>
impl From<Cerebrum<MarketUpdater>> for Cerebrum<OrderGenerator<Algorithmic>> {
    fn from(cerebrum: Cerebrum<MarketUpdater>) -> Self {
        Self {
            state: OrderGenerator { state: Algorithmic },
            feed: cerebrum.feed,
            event_tx: cerebrum.event_tx,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
        }
    }
}
