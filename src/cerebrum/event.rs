use super::order::{Cancelled, Open, Order};
use barter_data::model::MarketEvent;
use barter_integration::model::{Exchange, Instrument, Side, Symbol};
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use crate::cerebrum::exchange::ClientStatus;
use crate::execution::error::ExecutionError;

#[derive(Debug)]
pub enum Event {
    Market(MarketEvent),
    Account(AccountEvent),
    Command(Command),
}

#[derive(Debug)]
pub struct AccountEvent {
    pub exchange_time: DateTime<Utc>,
    pub received_time: DateTime<Utc>,
    pub exchange: Exchange,
    pub kind: AccountEventKind,
}

#[derive(Debug)]
pub enum AccountEventKind {
    ConnectionStatus(ClientStatus),
    Balance(SymbolBalance),
    Balances(Vec<SymbolBalance>),
    OrderNew(Order<Open>),
    OrderCancelled(Order<Cancelled>),
    OrdersOpen(Vec<Order<Open>>),
    Trade(Trade),
    ExecutionError(ExecutionError),
}

#[derive(Debug)]
pub enum Command {
    Terminate,
    FetchOpenPositions,
    ExitPosition,
    ExitAllPositions,
}

pub struct EventFeed {
    pub event_rx: mpsc::UnboundedReceiver<Event>
}

impl EventFeed {
    pub fn new(event_rx: mpsc::UnboundedReceiver<Event>) -> Self {
        Self { event_rx }
    }

    pub fn next(&mut self) -> Event {
        loop {
            match self.event_rx.try_recv() {
                Ok(event) => break event,
                Err(mpsc::error::TryRecvError::Empty) => continue,
                Err(mpsc::error::TryRecvError::Disconnected) => panic!("todo"),
            }
        }
    }
}

#[derive(Debug)]
pub struct Trade {
    pub id: TradeId,
    pub order_id: String,
    pub instrument: Instrument,
    pub side: Side,
    pub price: f64,
    pub amount: f64,
}

#[derive(Debug)]
pub struct TradeId(pub String);

#[derive(Debug)]
pub struct SymbolBalance {
    pub symbol: Symbol,
    pub balance: Balance,
}

#[derive(Clone, Copy, Debug)]
pub struct Balance {
    pub total: f64,
    pub available: f64,
}
