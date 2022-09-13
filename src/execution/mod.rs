use std::collections::HashMap;
use barter_integration::model::Exchange;
use tokio::sync::mpsc;
use crate::cerebrum::event::Event;
use crate::cerebrum::exchange::{ExchangeClient, ExchangeRequest};
use crate::execution::request::RequestFeed;

pub mod error;
mod request;

// Todo:
//  - Use RequestFeed? -> Would be a good way to determine if we've terminated
//  - Do I want a ClientStatus_rx? Maybe later. Ignore for now.
//  - ExecutionManager?

pub enum ExecutionEngine<Client> {
    RequestConsumer(ExchangeManager<RequestConsumer, Client>),
    Terminated(ExchangeManager<Terminated, Client>)
}

impl<Client> ExecutionEngine<Client>
where
    Client: ExchangeClient
{
    pub fn run(mut self) {
        'execution: loop {
            // Transition to next execution state
            self = self.next();

            if let Self::Terminated(_) = self {
                break 'execution
            }
        }
    }

    pub fn next(mut self) -> Self {
        match self {
            ExecutionEngine::RequestConsumer(manager) => {
                manager.next_request()
            }
            ExecutionEngine::Terminated(manager) => {
                Self::Terminated(manager)
            }
        }
    }

}

pub struct ExchangeManager<State, Client> {
    pub state: State,
    pub clients: HashMap<Exchange, Client>,
    pub feed: RequestFeed,
    pub event_tx: mpsc::UnboundedSender<Event>,
}

impl<State, Client> ExchangeManager<State, Client> {
    /// Retrieve the [`ExchangeClient`] associated with the [`Exchange`].
    pub fn client(&mut self, exchange: &Exchange) -> &Client {
        self.clients
            .get(exchange)
            .expect("cannot retrieve ExchangeClient for unexpected Exchange")
    }
}

pub struct RequestConsumer;

impl<Client> ExchangeManager<RequestConsumer, Client> {
    pub fn next_request(mut self)  -> ExecutionEngine<Client> {
        // Consume next execution Request
        match self.feed.next() {
            ExchangeRequest::FetchOpenOrders(exchanges) => {

            }
            ExchangeRequest::FetchBalances(exchanges) => {

            }
            ExchangeRequest::OpenOrders(open_requests) => {

            }
            ExchangeRequest::CancelOrders(cancel_requests) => {

            }
            ExchangeRequest::CancelOrdersAll(exchanges) => {

            }
        }
    }
}

pub struct Terminated;
























