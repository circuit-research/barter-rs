use crate::{
    event::Event,
    execution::{build_account_event, ExecutionRequest}
};
use barter_integration::model::Exchange;
use barter_execution::{
    ExecutionClient,
    model::AccountEventKind
};
use tokio::sync::mpsc;
use tracing::info;

pub struct ExecutionManager<Client>
where
    Client: ExecutionClient
{
    pub exchange: Exchange,
    pub execution_rx: mpsc::UnboundedReceiver<ExecutionRequest>,
    pub client: Client,
    pub event_feed_tx: mpsc::UnboundedSender<Event>,
}

impl<Client> ExecutionManager<Client>
where
    Client: ExecutionClient
{
    pub async fn run(mut self) {
        while let Some(request) = self.execution_rx.recv().await {
            let response_kind = self.execute(request).await;
            let event = build_account_event(self.exchange.clone(), response_kind);

            if self.event_feed_tx.send(event).is_err() {
                info!(
                    exchange = ?self.exchange,
                    "shutting down ExchangeSingle ExecutionClient manager"
                );
                break;
            }
        }
    }

    pub async fn execute(&self, request: ExecutionRequest) -> AccountEventKind {
        match request {
            ExecutionRequest::FetchOrdersOpen(exchange) => {
                self.validate(&exchange);
                AccountEventKind::OrdersOpen(self.client.fetch_orders_open().await)
            }
            ExecutionRequest::FetchOrdersOpenAll => {
                AccountEventKind::OrdersOpen(self.client.fetch_orders_open().await)
            }
            ExecutionRequest::OpenOrders(open_requests) => {
                AccountEventKind::OrdersNew(self.client.open_orders(open_requests).await)
            }
            ExecutionRequest::CancelOrders(cancel_requests) => {
                AccountEventKind::OrdersCancelled(self.client.cancel_orders(cancel_requests).await)
            }
            ExecutionRequest::CancelOrdersAll => {
                AccountEventKind::OrdersCancelled(self.client.cancel_orders_all().await)
            }
            ExecutionRequest::FetchBalances(exchange) => {
                self.validate(&exchange);
                AccountEventKind::Balances(self.client.fetch_balances().await)
            }
            ExecutionRequest::FetchBalancesAll => {
                AccountEventKind::Balances(self.client.fetch_balances().await)
            }
        }
    }

    pub fn validate(&self, exchange: &Exchange) {
        if self.exchange != *exchange {
            panic!("ExchangeSingle received ExecutionRequest with invalid exchange: {}", exchange)
        }
    }
}
