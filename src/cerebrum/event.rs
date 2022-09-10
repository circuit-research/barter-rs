use barter_data::model::MarketEvent;
use tokio::sync::mpsc;

pub enum Event {
    Market(MarketEvent),
    Account(AccountEvent),
    Command(Command),
}

pub enum AccountEvent { // Todo: Perhaps struct with ExchangeId at the top level and timestamp, etc.
    OrderNew,
    OrderCancelled,
    Trade,
    Balances,
    ConnectionStatus(ConnectionStatus),
}

#[derive(Clone, Copy, Debug)]
pub enum ConnectionStatus {
    Connected,
    CancelOnly,
    Disconnected,
}
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
