use std::collections::HashMap;
use barter_integration::model::Exchange;
use chrono::Utc;
use futures::StreamExt;
use tokio::sync::mpsc;
use crate::cerebrum::event::{AccountEvent, AccountEventKind, Event};
use crate::cerebrum::exchange::{ExchangeClient, ExecutionRequest};
use crate::cerebrum::order::{Open, Order};
use crate::execution::error::ExecutionError;
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
    pub clients: HashMap<Exchange, dyn Client>,
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
pub struct Terminated;

impl<Client> ExchangeManager<RequestConsumer, Client>
where
    Client: ExchangeClient
{
    // Todo:
    //  - Handling these requests would likely make new states
    //  - Do I want to make this async:
    //    '--> Use RwLock<HashMap<Exchange, dyn Client>
    //    '--> Fire and forget futures that get the data and send it on the event_tx (.send uses &self)
    pub fn next_request(mut self)  -> ExecutionEngine<Client> {
        // Consume next execution Request
        match self.feed.next() {
            ExecutionRequest::FetchOrdersOpen(exchanges) => {

            }
            ExecutionRequest::FetchBalances(exchanges) => {

            }
            ExecutionRequest::OpenOrders(open_requests) => {
                // Definitely should return Vec<Result<>>
            }
            ExecutionRequest::CancelOrders(cancel_requests) => {
                // Definitely should return Vec<Result<>>
            }
            ExecutionRequest::CancelOrdersAll(exchanges) => {
                // Definitely should return Vec<Result<>>
            }
        }

        ExecutionEngine::RequestConsumer(self)
    }

    /// Fetch every open [`Order`] on every every [`Exchange`], and send the aggregated collection
    /// over the [`Event`] transmitter.
    pub fn fetch_orders_open(&mut self, exchanges: Vec<Exchange>) {
        futures::stream::iter(exchanges)
            .for_each_concurrent(None, |exchange| {

                let kind = self.client(&exchange)
                    .fetch_orders_open()
                    .await
                    .map(AccountEventKind::OrdersOpen)
                    .unwrap_or_else(AccountEventKind::ExecutionError);

                // Todo:
                // - AccountEvent probably needs to change to Enum
                let account_event = AccountEvent {
                    exchange_time: Default::default(),
                    received_time: Utc::now(),
                    exchange,
                    kind: kind
                };

                self.event_tx
                    .send(Event::Account(account_event))
                    .unwrap();
            });

        // for exchange in exchanges {
        // }

        let client = self.client(&exchange);

        let exchange_orders = client
            .fetch_orders_open()
            .await;

        self.event_tx
            .send(Event::Account())


    }
}


























