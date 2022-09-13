use tokio::sync::mpsc;
use crate::cerebrum::exchange::ExchangeRequest;

pub struct RequestFeed {
    pub request_tx: mpsc::UnboundedReceiver<ExchangeRequest>
}

impl RequestFeed {
    pub fn new(request_tx: mpsc::UnboundedReceiver<ExchangeRequest>) -> Self {
        Self { request_tx }
    }

    pub fn next(&mut self) -> ExchangeRequest {
        loop {
            match self.request_tx.try_recv() {
                Ok(event) => break event,
                Err(mpsc::error::TryRecvError::Empty) => continue,
                Err(mpsc::error::TryRecvError::Disconnected) => panic!("todo"),
            }
        }
    }
}