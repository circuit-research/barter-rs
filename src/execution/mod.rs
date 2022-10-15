// use request::RequestFeed;
// use crate::{
//     event::Event,
// };
// use barter_integration::model::Exchange;
// use barter_execution::ExecutionClient;
// use std::collections::HashMap;
// use chrono::Utc;
// use futures::StreamExt;
// use tokio::sync::mpsc;
// use barter_execution::model::{AccountEvent, AccountEventKind};
// use barter_execution::model::order::{Order, RequestCancel, RequestOpen};
//
// pub mod request;
//
// // Todo:
// //  - Do I want a ClientStatus_rx? Maybe later. Ignore for now.
// //  - ExecutionManager?
//
// pub enum ExecutionEngine<Client> {
//     RequestConsumer(ExchangeManager<RequestConsumer, Client>),
//     Terminated(ExchangeManager<Terminated, Client>)
// }
//
// impl<Client> ExecutionEngine<Client>
// where
//     Client: ExecutionClient
// {
//     pub fn run(mut self) {
//         'execution: loop {
//             // Transition to next execution state
//             self = self.next();
//
//             if let Self::Terminated(_) = self {
//                 break 'execution
//             }
//         }
//     }
//
//     pub fn next(mut self) -> Self {
//         match self {
//             ExecutionEngine::RequestConsumer(manager) => {
//                 manager.next_request()
//             }
//             ExecutionEngine::Terminated(manager) => {
//                 Self::Terminated(manager)
//             }
//         }
//     }
//
// }
//
// pub struct ExchangeManager<State, Client> {
//     pub state: State,
//     pub clients: HashMap<Exchange, dyn Client>,
//     pub feed: RequestFeed,
//     pub event_tx: mpsc::UnboundedSender<Event>,
// }
//
// impl<State, Client> ExchangeManager<State, Client> {
//     /// Retrieve the [`ExchangeClient`] associated with the [`Exchange`].
//     pub fn client(&mut self, exchange: &Exchange) -> &Client {
//         self.clients
//             .get(exchange)
//             .expect("cannot retrieve ExchangeClient for unexpected Exchange")
//     }
// }
//
// pub struct RequestConsumer;
// pub struct Terminated;
//
// impl<Client> ExchangeManager<RequestConsumer, Client>
// where
//     Client: ExecutionClient
// {
//     // Todo:
//     //  - Handling these requests would likely make new states
//     //  - Do I want to make this async:
//     //    '--> Use RwLock<HashMap<Exchange, dyn Client>
//     //    '--> Fire and forget futures that get the data and send it on the event_tx (.send uses &self)
//     pub async fn next_request(mut self)  -> ExecutionEngine<Client> {
//         // Consume next execution Request
//         match self.feed.next() {
//             ExecutionRequest::FetchBalances(exchanges) => {
//                 self.fetch_balances(exchanges).await;
//             }
//             ExecutionRequest::FetchOrdersOpen(exchanges) => {
//                 self.fetch_orders_open(exchanges).await;
//             }
//             ExecutionRequest::OpenOrders(open_requests) => {
//                 self.open_orders(open_requests).await;
//             }
//             ExecutionRequest::CancelOrders(cancel_requests) => {
//                 self.cancel_orders(cancel_requests).await;
//             }
//             ExecutionRequest::CancelOrdersAll(exchanges) => {
//                 // Definitely should return Vec<Result<>>
//             }
//         }
//
//         ExecutionEngine::RequestConsumer(self)
//     }
//
//     /// Fetch every open [`Order`] on every every [`Exchange`], and send the aggregated collection
//     /// over the [`Event`] transmitter.
//     pub async fn fetch_orders_open(&mut self, exchanges: Vec<Exchange>) {
//         futures::stream::iter(exchanges)
//             .for_each_concurrent(None, |exchange| {
//                 let kind = self
//                     .client(&exchange)
//                     .fetch_orders_open()
//                     .await
//                     .map(AccountEventKind::OrdersOpen)
//                     .unwrap_or_else(AccountEventKind::ExecutionError);
//
//                 let account_event = AccountEvent { received_time: Utc::now(), exchange, kind };
//
//                 self.event_tx
//                     .send(Event::Account(account_event))
//                     .unwrap();
//             });
//     }
//
//     pub async fn open_orders(&mut self, open_requests: Vec<(Exchange, Vec<Order<RequestOpen>>)>) {
//         futures::stream::iter(open_requests)
//             .for_each_concurrent(None, |(exchange, open_requests)| {
//                 let kind = self
//                     .client(&exchange)
//                     .open_orders(open_requests)
//                     .await
//                     .map(AccountEventKind::OrdersNew)
//                     .unwrap_or_else(AccountEventKind::ExecutionError);
//
//                 let account_event = AccountEvent { received_time: Utc::now(), exchange, kind };
//
//                 self.event_tx
//                     .send(Event::Account(account_event))
//                     .unwrap();
//             });
//     }
//
//     pub async fn cancel_orders(&mut self, cancel_requests: Vec<(Exchange, Vec<Order<RequestCancel>>)>) {
//         futures::stream::iter(cancel_requests)
//             .for_each_concurrent(None, |(exchange, cancel_requests)| {
//                 let kind = self
//                     .client(&exchange)
//                     .cancel_orders(cancel_requests)
//                     .await
//                     .map(AccountEventKind::OrdersCancelled)
//                     .unwrap_or_else(AccountEventKind::ExecutionError);
//
//                 let account_event = AccountEvent { received_time: Utc::now(), exchange, kind };
//
//                 self.event_tx
//                     .send(Event::Account(account_event))
//                     .unwrap();
//             });
//     }
//
//     pub async fn cancel_orders_all(&mut self, exchanges: Vec<Exchange>) {
//         futures::stream::iter(exchanges)
//             .for_each_concurrent(None, |exchange| {
//                 let kind = self
//                     .client(&exchange)
//                     .cancel_orders_all()
//                     .await
//                     .map(AccountEventKind::OrdersCancelled)
//                     .unwrap_or_else(AccountEventKind::ExecutionError);
//
//                 let account_event = AccountEvent { received_time: Utc::now(), exchange, kind };
//
//                 self.event_tx
//                     .send(Event::Account(account_event))
//                     .unwrap();
//             });
//     }
// }
//
//
//
//
//
//




















