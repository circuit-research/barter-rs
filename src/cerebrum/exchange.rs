use super::{
    event::{ConnectionStatus, SymbolBalance},
    account::ClientOrderId,
    order::{Order, RequestCancel, RequestOpen},
};
use barter_integration::model::{Exchange, Instrument};
use std::{
    collections::HashMap,
    time::Duration
};


// Todo:
//  - May need to have an synchronous interface prior to async for eg/ GenerateClientOrderId
#[derive(Debug)]
pub enum ExchangeRequest {
    // Check connection status
    ConnectionStatus,

    // Fetch Account State
    FetchOpenOrders,
    FetchBalances,

    // Open Orders
    // OpenOrder(Order<RequestOpen>),
    // OpenOrderBatch(Order<Vec<RequestOpen>>),
    OpenOrders(Vec<Order<RequestOpen>>),

    // Cancel Orders
    // CancelOrderById,
    // CancelOrderByInstrument,
    // CancelOrderByBatch,
    CancelOrders(Vec<Order<RequestCancel>>),
    CancelOrdersAll(Vec<Exchange>),
}

pub trait ExchangeClient {
    fn instruments(&self) -> &[Instrument];
    fn connection_status(&self) -> ConnectionStatus;

    fn fetch_orders_open(&self) -> ();
    fn fetch_balances(&self) -> ();

    fn open_order(&self) -> ();
    fn open_order_batch(&self) -> ();

    fn cancel_order_by_id(&self) -> ();
    fn cancel_order_by_instrument(&self) -> ();
    fn cancel_order_by_batch(&self) -> ();
    fn cancel_order_all(&self) -> ();
}

pub struct SimulatedExchange {
    pub meta: SimulationMeta,
    pub latency: Duration,
    pub instruments: Vec<Instrument>,
    pub connection_status: ConnectionStatus,
    pub open: HashMap<ClientOrderId, ()>,
    pub balances: HashMap<Instrument, SymbolBalance>,
}

pub struct SimulationMeta {
    fees: (),
    latency: Duration,
}

impl ExchangeClient for SimulatedExchange {
    fn instruments(&self) -> &[Instrument] {
        &self.instruments
    }

    fn connection_status(&self) -> ConnectionStatus {
        self.connection_status
    }

    fn fetch_orders_open(&self) -> () {

    }

    fn fetch_balances(&self) -> () {
        todo!()
    }

    fn open_order(&self) -> () {
        todo!()
    }

    fn open_order_batch(&self) -> () {
        todo!()
    }

    fn cancel_order_by_id(&self) -> () {
        todo!()
    }

    fn cancel_order_by_instrument(&self) -> () {
        todo!()
    }

    fn cancel_order_by_batch(&self) -> () {
        todo!()
    }

    fn cancel_order_all(&self) -> () {
        todo!()
    }
}