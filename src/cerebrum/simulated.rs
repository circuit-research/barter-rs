use super::{
    event::{Balance, Event},
    exchange::{ExecutionClient, ClientOrderId, ClientStatus},
    order::{Order, Open},
};
use barter_integration::model::{Instrument, Symbol};
use std::{
    time::Duration,
    collections::HashMap
};
use tokio::sync::{
    mpsc, mpsc::UnboundedSender
};
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct Config {
    instruments: Vec<Instrument>,
    starting_balance: Balance,
    fees: (),
    latency: Duration,
}

#[derive(Clone, Debug)]
pub struct SimulatedExchange {
    pub config: Config,
    pub connection_status: ClientStatus,
    pub event_tx: mpsc::UnboundedSender<Event>,
    pub balances: HashMap<Symbol, Balance>,
    pub open: HashMap<ClientOrderId, Order<Open>>,
}

#[async_trait]
impl ExecutionClient for SimulatedExchange {
    type Config = Config;

    async fn init(config: Self::Config, event_tx: UnboundedSender<Event>) -> Self {
        // Assign each Symbol the provided starting Balance
        let balances = config
            .instruments
            .iter()
            .map(|instrument| [&instrument.base, &instrument.quote])
            .flatten()
            .map(|symbol| (symbol.clone(), config.starting_balance.clone()))
            .collect();

        Self {

            config,
            connection_status: ClientStatus::Connected,
            event_tx,
            balances,
            open: HashMap::new(),
        }
    }

    async fn consume(&self, event_tx: UnboundedSender<Event>) -> Result<(), ()> {
        // Spawn WebSocket connection that sends updates
        Ok(())
    }

    fn connection_status(&self) -> ClientStatus {
        todo!()
    }

    async fn fetch_orders_open(&self) -> () {
        todo!()
    }

    async fn fetch_balances(&self) -> () {
        todo!()
    }

    async fn open_order(&self) -> () {
        todo!()
    }

    async fn open_order_batch(&self) -> () {
        todo!()
    }

    async fn cancel_order_by_id(&self) -> () {
        todo!()
    }

    async fn cancel_order_by_instrument(&self) -> () {
        todo!()
    }

    async fn cancel_order_by_batch(&self) -> () {
        todo!()
    }

    async fn cancel_orders_all(&self) -> () {
        todo!()
    }
}