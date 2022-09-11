use std::collections::HashMap;
use std::time::Duration;
use barter_integration::model::{Instrument, Symbol};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use barter_execution::model::{ClientOrderId, ConnectionStatus};
use barter_execution::simulated::Order;
use crate::cerebrum::event::{Balance, Event};
use crate::cerebrum::exchange::ExchangeClient;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct Config {
    instruments: Vec<Instrument>,
    starting_balance: Balance,
    fees: (),
    latency: Duration,
}

#[derive(Clone, Debug)]
pub struct SimulatedExchange<'a> {
    pub config: Config,
    pub connection_status: ConnectionStatus,
    pub event_tx: mpsc::UnboundedSender<Event>,
    pub balances: HashMap<&'a Symbol, Balance>,
    pub open: HashMap<ClientOrderId, Order>,
}

#[async_trait]
impl ExchangeClient for SimulatedExchange<'_> {
    type Config = Config;

    async fn init(config: Self::Config, event_tx: UnboundedSender<Event>) -> Self {
        // Assign each Symbol the provided starting Balance
        let balances = config
            .instruments
            .iter()
            .map(|instrument| [&instrument.base, &instrument.quote])
            .flatten()
            .map(|symbol| (symbol, config.starting_balance.clone()))
            .collect();

        Self {
            config,
            connection_status: ConnectionStatus::Connected,
            event_tx,
            balances,
            open: HashMap::new(),
        }
    }

    async fn consume(&self, event_tx: UnboundedSender<Event>) -> Result<(), ()> {
        // Spawn WebSocket connection that sends updates
        Ok(())
    }

    fn instruments(&self) -> &[Instrument] {
        todo!()
    }

    fn connection_status(&self) -> ConnectionStatus {
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

    async fn cancel_order_all(&self) -> () {
        todo!()
    }
}