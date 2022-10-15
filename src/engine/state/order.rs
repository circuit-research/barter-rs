use super::{
    consume::Consume,
    terminate::Terminate,
};
use crate::{
    engine::{
        Engine, Trader,
        error::EngineError,
    },
    strategy::OrderGenerator,
    portfolio::{AccountUpdater, MarketUpdater},
    execution::ExecutionRequest,
};
use barter_execution::model::order::{Order, RequestOpen};

/// [`GenerateOrder`] can only transition to:
/// a) [`Consume`]
/// b) [`Terminate`]
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
    pub fn generate_order_requests(self) -> Engine<Strategy, Portfolio> {
        // Send CancelOrders ExecutionRequest to ExecutionClient
        if let Some(cancel_requests) = self.strategy.generate_cancels() {
            if self.execution_tx
                .send(ExecutionRequest::CancelOrders(cancel_requests))
                .is_err()
            {
                // Transition to Engine state Terminate since ExecutionRequest receiver dropped
                return Engine::Terminate(Trader::from(self))
            }
        }

        // Send OpenOrders ExecutionRequest to ExecutionClient
        if let Some(open_requests) = self.strategy.generate_orders() {
            // If the ExecutionRequest receiver is dropped we must terminate
            if self.execution_tx
                .send(ExecutionRequest::OpenOrders(open_requests))
                .is_err()
            {
                // Transition to Engine state Terminate since ExecutionRequest receiver dropped
                return Engine::Terminate(Trader::from(self))
            }
        }

        Engine::Consume(Trader::from(self))
    }
}

impl<Strategy, Portfolio> Trader<GenerateOrder<Portfolio, Manual>, Strategy>
where
    Strategy: OrderGenerator,
    Portfolio: MarketUpdater + AccountUpdater,
{
    pub fn generate_order_requests_manual(self, _order: Order<RequestOpen>) -> Engine<Strategy, Portfolio> {
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

/// a) GenerateOrder<State> -> Terminate
impl<Strategy, Portfolio, Kind> From<Trader<Strategy, GenerateOrder<Portfolio, Kind>>> for Trader<Strategy, Terminate<Portfolio>> {
    fn from(trader: Trader<Strategy, GenerateOrder<Portfolio, Kind>>) -> Self {
        Self {
            feed: trader.feed,
            strategy: trader.strategy,
            execution_tx: trader.execution_tx,
            state: Terminate {
                portfolio: Some(trader.state.portfolio),
                reason: Err(EngineError::ExecutionTerminated)
            }
        }
    }
}