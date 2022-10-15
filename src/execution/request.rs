use crate::event::Feed;
use tokio::sync::mpsc;

pub enum ExecutionRequest {

}

pub struct RequestFeed {
    pub request_rx: mpsc::UnboundedReceiver<ExecutionRequest>
}

impl RequestFeed {
    pub fn new(request_rx: mpsc::UnboundedReceiver<ExecutionRequest>) -> Self {
        Self { request_rx }
    }

    pub fn next(&mut self) -> Feed<ExecutionRequest> {
        loop {
            match self.request_rx.try_recv() {
                Ok(request) => break Feed::Next(request),
                Err(mpsc::error::TryRecvError::Empty) => continue,
                Err(mpsc::error::TryRecvError::Disconnected) => break Feed::Finished
            }
        }
    }
}

