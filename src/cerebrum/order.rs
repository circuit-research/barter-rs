use super::{
    Engine, Cerebrum,
    consume::Consumer,
    account::ClientOrderId,
    exchange::ExchangeRequest
};
use barter_integration::model::{Exchange, Side};

/// OrderGenerator can transition to:
///  a) Consumer
pub struct OrderGenerator<State> {
    pub state: State,
}

pub struct Algorithmic;
pub struct Manual;

impl<Strategy> Cerebrum<OrderGenerator<Algorithmic>, Strategy>
where
    Strategy: super::strategy::OrderGenerator,
{
    pub fn generate_order_requests(mut self) -> Engine<Strategy> {
        // Send CancelOrders Command to ExchangeClient
        if let Some(cancel_requests) = self.strategy.generate_cancels() {
            self.exchange_tx
                .send(ExchangeRequest::CancelOrders(cancel_requests))
                .unwrap()
        }

        // Send OpenOrders Command to ExchangeClient
        if let Some(open_requests) = self.strategy.generate_orders() {
            self.exchange_tx
                .send(ExchangeRequest::OpenOrders(open_requests))
                .unwrap();
        }

        Engine::Consumer(Cerebrum::from(self))
    }
}

impl<Strategy> Cerebrum<OrderGenerator<Manual>, Strategy> {
    pub fn generate_order_requests_manual(mut self, meta: ()) -> Engine<Strategy> {
        // Todo:
        // 1. Action manual open / cancel order
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
pub struct RequestOpen {
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
pub struct RequestCancel {
    pub kind: OrderKind,
    pub side: Side,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Clone, Debug)]
pub struct InFlightCancel;

#[derive(Clone, Debug)]
pub struct Cancelled;

#[derive(Clone, Copy, Debug)]
pub enum OrderKind {
    Market,
    Limit,
    PostOnly,
    ImmediateOrCancel
}
