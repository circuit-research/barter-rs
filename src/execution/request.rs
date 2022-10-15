use crate::event::Feed;
use barter_integration::model::Exchange;
use barter_execution::model::order::{Order, RequestOpen};
use tokio::sync::mpsc;

pub enum ExecutionRequest {
    FetchOrdersOpen(Exchange),
    FetchOrdersOpenAll,
    OpenOrders(Vec<Order<RequestOpen>>),
    CancelOrders(Exchange),
    CancelOrdersAll,
    FetchBalances(Exchange),
    FetchBalancesAll,
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

