use crate::cerebrum::Engine;
use super::{
    Cerebrum,
    consume::Consumer,
};

/// OrderGenerator can transition to:
///  a) Consumer
pub struct OrderGenerator<State> {
    pub state: State,
}

pub struct Algorithmic;
pub struct Manual { pub meta: () }

impl Cerebrum<OrderGenerator<Algorithmic>> {
    pub fn generate_order(mut self) -> Engine {
        // 1. Analyse open Positions, Orders, Statistics, Indicators
        // 2. Decide whether to cancel or open orders
        // 3. Action the decisions
        todo!();

        Engine::Consumer(Cerebrum::from(self))
    }
}

impl Cerebrum<OrderGenerator<Manual>> {
    pub fn generate_order_manual(mut self) -> Engine {
        // 1. Analyse open Positions, Orders, Statistics, Indicators
        // 2. Decide whether to cancel or open orders
        // 3. Action the decisions
        todo!();

        Engine::Consumer(Cerebrum::from(self))
    }
}

/// a) OrderGenerator -> Consumer
impl<State> From<Cerebrum<OrderGenerator<State>>> for Cerebrum<Consumer> {
    fn from(cerebrum: Cerebrum<OrderGenerator<State>>) -> Self {
        Self {
            state: Consumer,
            feed: cerebrum.feed,
            event_tx: cerebrum.event_tx,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
        }
    }
}
