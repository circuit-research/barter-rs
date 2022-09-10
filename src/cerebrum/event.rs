use barter_data::model::MarketEvent;
use barter_integration::model::{Exchange, Symbol};
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;

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
    OrderNew,
    OrderCancelled,
    Trade,
    Balance(SymbolBalance),
    Balances(Vec<SymbolBalance>),
    ConnectionStatus(ConnectionStatus),
}

#[derive(Clone, Copy, Debug)]
pub enum ConnectionStatus {
    Connected,
    CancelOnly,
    Disconnected,
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
pub struct SymbolBalance {
    pub symbol: Symbol,
    pub balance: Balance,
}

#[derive(Clone, Copy, Debug)]
pub struct Balance {
    pub total: f64,
    pub available: f64,
}
