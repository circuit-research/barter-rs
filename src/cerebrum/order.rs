use super::{
    Cerebrum,
    consumer::Consumer,
};

/// OrderGenerator can transition to:
///  a) Consumer
pub struct OrderGenerator<State> {
    pub state: State,
}

pub struct Algorithmic;
pub struct Manual { pub meta: () }

impl Cerebrum<OrderGenerator<Algorithmic>> {
    fn generate(mut self) -> Cerebrum<Consumer> {
        // 1. Analyse open Positions, Orders, Statistics, Indicators
        // 2. Decide whether to cancel or open orders
        // 3. Action the decisions
        todo!();

        Cerebrum::from(self)
    }
}

/// a) OrderGenerator -> Consumer
impl<State> From<Cerebrum<OrderGenerator<State>>> for Cerebrum<Consumer> {
    fn from(cerebrum: Cerebrum<OrderGenerator<State>>) -> Self {
        Self {
            feed: cerebrum.feed,
            event_tx: cerebrum.event_tx,
            event_q: cerebrum.event_q,
            balances: cerebrum.balances,
            orders: cerebrum.orders,
            positions: cerebrum.positions,
            strategy: cerebrum.strategy,
            state: Consumer
        }
    }
}
