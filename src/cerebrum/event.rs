use barter_data::model::MarketEvent;
use tokio::sync::mpsc;

pub enum Event {
    Market(MarketEvent),
    Account(AccountEvent),
    Command(Command),
}

pub enum AccountEvent {
    OrderNew,
    OrderCancelled,
    Trade,
    Balances
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
            println!("TryRecv");
            match self.event_rx.try_recv() {
                Ok(event) => break event,
                Err(mpsc::error::TryRecvError::Empty) => continue,
                Err(mpsc::error::TryRecvError::Disconnected) => panic!("todo"),
            }
        }
    }
}
