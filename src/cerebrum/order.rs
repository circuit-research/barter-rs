use crate::cerebrum::{Cerebrum, CerebrumState};
use crate::cerebrum::consumer::Consumer;


/// OrderGenerator can transition to:
///  a) Consumer
pub struct OrderGenerator;

impl Cerebrum<OrderGenerator> {
    fn generate(mut self) -> Cerebrum<Consumer> {
        // 1. Analyse open Positions, Orders, Statistics, Indicators
        // 2. Decide whether to cancel or open orders
        // 3. Action the decisions
        todo!();

        Cerebrum::from(self)
    }
}

/// a) OrderGenerator -> Consumer
impl From<Cerebrum<OrderGenerator>> for Cerebrum<Consumer> {
    fn from(cerebrum: Cerebrum<OrderGenerator>) -> Self {
        Self {
            feed: cerebrum.feed,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: Consumer
        }
    }
}
