use barter_data::model::MarketEvent;
use barter_execution::model::AccountEvent;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

/// Communicates the state of the [`Feed`] as well as the next event.
#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub enum Feed<Event> {
    Next(Event),
    Finished,
}

#[derive(Debug, Clone)]
pub enum Event {
    Market(MarketEvent),
    Account(AccountEvent),
    Command(Command),
}

#[derive(Debug, Clone)]
pub enum Command {
    FetchOpenPositions,
    ExitPosition,
    ExitAllPositions,
    Terminate,
}

pub struct EventFeed {
    pub event_rx: mpsc::UnboundedReceiver<Event>,
}

impl EventFeed {
    pub fn new(event_rx: mpsc::UnboundedReceiver<Event>) -> Self {
        Self { event_rx }
    }

    pub fn next(&mut self) -> Feed<Event> {
        loop {
            match self.event_rx.try_recv() {
                Ok(event) => break Feed::Next(event),
                Err(mpsc::error::TryRecvError::Empty) => continue,
                Err(mpsc::error::TryRecvError::Disconnected) => break Feed::Finished,
            }
        }
    }
}


