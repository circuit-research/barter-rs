use super::{
    Engine, Cerebrum,
    consume::Consumer,
};

/// OrderGenerator can transition to:
///  a) Consumer
pub struct OrderGenerator<State> {
    pub state: State,
}

pub struct Algorithmic;
pub struct Manual { pub meta: () }

impl<Strategy> Cerebrum<OrderGenerator<Algorithmic>, Strategy> {
    pub fn generate_order(mut self) -> Engine<Strategy> {
        // Todo:
        // 1. Analyse open Positions, Orders, Statistics, Indicators
        // 2. Decide whether to cancel or open orders
        // 3. Action the decisions
        Engine::Consumer(Cerebrum::from(self))
    }
}

impl<Strategy> Cerebrum<OrderGenerator<Manual>, Strategy> {
    pub fn generate_order_manual(mut self) -> Engine<Strategy> {
        // Todo:
        // 1. Analyse open Positions, Orders, Statistics, Indicators
        // 2. Decide whether to cancel or open orders
        // 3. Action the decisions
        Engine::Consumer(Cerebrum::from(self))
    }
}

/// a) OrderGenerator -> Consumer
impl<State, Strategy> From<Cerebrum<OrderGenerator<State>, Strategy>> for Cerebrum<Consumer, Strategy> {
    fn from(cerebrum: Cerebrum<OrderGenerator<State>, Strategy>) -> Self {
        Self {
            state: Consumer,
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}
