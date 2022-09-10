use super::{Balance, ClientOrderId, event::ConnectionStatus};
use barter_integration::model::{Instrument, Market};
use std::{
    time::Duration,
    collections::HashMap
};

// Todo: May need to have an synchronous interface prior to async for eg/ GenerateClientOrderId
pub enum ExchangeCommand {
    // Check connection status
    ConnectionStatus,

    // Fetch Account State
    FetchOpenOrders,
    FetchBalances,

    // Open Orders
    OpenOrder,
    OpenOrderBatch,

    // Cancel Orders
    CancelOrderById,
    CancelOrderByInstrument,
    CancelOrderByBatch,
    CancelOrderAll,
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
    pub balances: HashMap<Instrument, Balance>,
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