use super::{
    event::Event,
    order::{Order, RequestCancel, RequestOpen},
};
use barter_integration::model::{Exchange, Instrument};
use std::{
    collections::HashMap,
};
use tokio::sync::mpsc;
use tracing::info;
use async_trait::async_trait;
use uuid::Uuid;

/// Responsibilities:
/// - Determines best way to action an [`ExchangeRequest`] given the constraints of the exchange.
#[async_trait]
pub trait ExchangeClient {
    type Config;

    async fn init(config: Self::Config, event_tx: mpsc::UnboundedSender<Event>) -> Self;
    async fn consume(&self, event_tx: mpsc::UnboundedSender<Event>) -> Result<(), ()>;

    fn instruments(&self) -> &[Instrument];
    fn connection_status(&self) -> ConnectionStatus;

    async fn fetch_orders_open(&self) -> ();
    async fn fetch_balances(&self) -> ();

    async fn open_order(&self) -> ();
    async fn open_order_batch(&self) -> ();

    async fn cancel_order_by_id(&self) -> ();
    async fn cancel_order_by_instrument(&self) -> ();
    async fn cancel_order_by_batch(&self) -> ();
    async fn cancel_order_all(&self) -> ();
}

/// Responsibilities:
/// - Manages every [`ExchangeClient`].
/// - Forwards an [`ExchangeRequest`] to the appropriate [`ExchangeClient`].
/// - Map InternalClientOrderId to exchange ClientOrderId.
pub struct ExchangePortal<Client>
where
    Client: ExchangeClient,
{
    clients: HashMap<Exchange, Client>,
    request_rx: mpsc::UnboundedReceiver<ExchangeRequest>,
    event_tx: mpsc::UnboundedSender<Event>,
}

impl<Client> ExchangePortal<Client>
where
    Client: ExchangeClient,
{

    pub fn init(
        exchanges: HashMap<Exchange, ClientId>,
        event_tx: mpsc::UnboundedSender<Event>
    ) -> Result<Self, ()> {
        // Todo:
        //  - Validate input
        //  - I don't think there is any reason the core would ask for ConnectionStatus, but it would be sent
        //  - Can ExchangePortal act as the Driver? Yes.
        //  - Make ExchangePortal state machine...

        // 1. Store HashMap<Exchange, ClientId> for association & to keep every ClientId(Config)
        // 2. Spawn tasks for every ExchangeClient
        // 3. Monitor ConnectionStatus of each task
        // 4. Re-spawn ExchangeClient if required

        for (exchange, client_id) in &exchanges {
            match client_id {
                ClientId::Simulated(config) => {





                    // Construct New Driver (will never fail)
                    // Driver spawns new Clients
                }
            }

            // Runner



        }



        Err(())
    }

    /// Todo:
    ///  - Should be run on it's own OS thread.
    ///  - This may live in Barter... ExchangeClient impls would live here. Order would be in Barter!
    ///  - Just use HTTP for trading for the time being...
    pub fn run(mut self) {
        loop {
            // Receive next ExchangeRequest
            let request = match self.request_rx.try_recv() {
                Ok(request) => request,
                Err(mpsc::error::TryRecvError::Empty) => continue,
                Err(mpsc::error::TryRecvError::Disconnected) => panic!("todo"),
            };
            info!(payload = ?request, "received ExchangeRequest");

            // Action ExchangeRequest
            match request {
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

    /// Retrieve the [`ExchangeClient`] associated with the [`Exchange`].
    pub fn client(&mut self, exchange: &Exchange) -> &Client {
        self.clients
            .get(exchange)
            .expect("cannot retrieve ExchangeClient for unexpected Exchange")
    }
}

#[derive(Debug)]
pub enum ExchangeRequest {
    // Check connection status
    // ConnectionStatus(Vec<Exchange>),

    // Fetch Account State
    FetchOpenOrders(Vec<Exchange>),
    FetchBalances(Vec<Exchange>),

    // Open Orders
    // OpenOrder(Order<RequestOpen>),
    // OpenOrderBatch(Order<Vec<RequestOpen>>),
    OpenOrders(Vec<Order<RequestOpen>>),

    // Cancel Orders
    // CancelOrderById,
    // CancelOrderByInstrument,
    // CancelOrderByBatch,
    CancelOrders(Vec<Order<RequestCancel>>),
    CancelOrdersAll(Vec<Exchange>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ClientOrderId(pub Uuid);

#[derive(Clone, Copy, Debug)]
pub enum ConnectionStatus {
    Connected,
    CancelOnly,
    Disconnected,
}

// Todo:
//   - Better name for this? This is the equivilant to ExchangeId...
//    '--> renamed to ClientId for now to avoid confusion in development
#[derive(Clone, Debug)]
pub enum ClientId {
    Simulated(super::simulated::Config),
    Binance(super::binance::Config)
}
