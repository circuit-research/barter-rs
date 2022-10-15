use crate::engine::{Engine, Trader};
use barter_execution::model::order::{Order, RequestOpen};
use crate::engine::state::consume::Consume;
use crate::execution::ExecutionRequest;
use crate::portfolio::{AccountUpdater, MarketUpdater};
use crate::strategy::OrderGenerator;

/// [`GenerateOrder`] can only transition to:
/// a) [`Consume`]
pub struct GenerateOrder<Portfolio, Kind> {
    pub portfolio: Portfolio,
    pub kind: Kind,
}

pub struct Algorithmic;
pub struct Manual;

impl<Strategy, Portfolio> Trader<Strategy, GenerateOrder<Portfolio, Algorithmic>>
where
    Strategy: OrderGenerator,
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn generate_order_requests(mut self) -> Engine<Strategy, Portfolio> {
        // // Send CancelOrders Command to ExchangeClient
        // if let Some(cancel_requests) = self.strategy.generate_cancels() {
        //     self.request_tx
        //         .send(ExecutionRequest::CancelOrders(cancel_requests))
        //         .unwrap()
        // }
        //
        // // Send OpenOrders Command to ExchangeClient
        // if let Some(open_requests) = self.strategy.generate_orders() {
        //     self.request_tx
        //         .send(ExecutionRequest::OpenOrders(open_requests))
        //         .unwrap();
        // }

        Engine::Consume(Trader::from(self))
    }
}

impl<Strategy, Portfolio> Trader<GenerateOrder<Portfolio, Manual>, Strategy>
where
    Strategy: OrderGenerator,
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn generate_order_requests_manual(mut self, order: Order<RequestOpen>) -> Engine<Strategy, Portfolio> {
        // Todo: Action manual open / cancel order
        // Engine::Consumer(Trader::from(self))
        unimplemented!()
    }
}

/// a) GenerateOrder<State> -> Consume
impl<Strategy, Portfolio, Kind> From<Trader<Strategy, GenerateOrder<Portfolio, Kind>>> for Trader<Strategy, Consume<Portfolio>> {
    fn from(trader: Trader<Strategy, GenerateOrder<Portfolio, Kind>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Consume {
                portfolio: trader.state.portfolio
            }
        }
    }
}