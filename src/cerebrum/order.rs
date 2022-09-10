use barter_integration::model::{Exchange, Side};
use crate::cerebrum::account::ClientOrderId;
use crate::cerebrum::exchange::ExchangeCommand;
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
pub struct Manual;

impl<Strategy> Cerebrum<OrderGenerator<Algorithmic>, Strategy> {
    pub fn generate_order(mut self) -> Engine<Strategy> {
        // Todo:
        // 1. Analyse open Positions, Orders, Statistics, Indicators
        // 2. Decide whether to cancel or open orders
        // 3. Action the decisions

        // let order_request = Order {
        //     exchange: (),
        //     cid: ClientOrderId(),
        //     state: ()
        // };
        // let order_request_batch = vec![
        //     Order {
        //         exchange: (),
        //         cid: ClientOrderId(),
        //         state: Request {
        //             kind: OrderKind::Market,
        //             side: Side::Buy,
        //             price: 0.0,
        //             quantity: 0.0
        //         }
        //     }
        // ];

        self.exchange_tx.send(ExchangeCommand::OpenOrder).unwrap();
        Engine::Consumer(Cerebrum::from(self))
    }
}

impl<Strategy> Cerebrum<OrderGenerator<Manual>, Strategy> {
    pub fn generate_order_manual(mut self, meta: ()) -> Engine<Strategy> {
        // Todo:
        // 1. Action manual open / close order

        self.exchange_tx.send(ExchangeCommand::OpenOrder).unwrap();
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
            audit_tx: cerebrum.audit_tx,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Order<State> {
    pub exchange: Exchange,
    pub cid: ClientOrderId,
    pub state: State,
}

#[derive(Clone, Debug)]
pub struct Request {
    pub kind: OrderKind,
    pub side: Side,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Clone, Debug)]
pub struct InFlight;

#[derive(Clone, Debug)]
pub struct Open {
    pub direction: Side,
    pub price: f64,
    pub quantity: f64,
    pub filled_quantity: f64,
}

#[derive(Clone, Debug)]
pub struct Cancelled;

#[derive(Clone, Copy, Debug)]
pub enum OrderKind {
    Market,
    Limit,
    PostOnly,
    ImmediateOrCancel
}
