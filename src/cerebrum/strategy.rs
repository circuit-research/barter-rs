use barter_data::model::MarketEvent;
use crate::cerebrum::order::{Cancelled, Order, Request};

pub trait IndicatorUpdater {
    fn update_indicators(&mut self, market: &MarketEvent);
}

// A batch Vec<Order<Request>> only makes sense for a single exchange, so would potentially generate
// several vectors for several batches...
// '--> all this would have to be atomic computation to ensure balances we fine :)
pub trait OrderGenerator {
    fn generate_orders() -> Option<Vec<Order<Request>>>;
    fn generate_cancels() -> Option<Vec<Order<Cancelled>>>; // Do we need a new another State eg/ InFlightCancelled
    // fn build_order() ->
}


// Todo: What does the Strategy do?
// - Updates Indicators
// - Analyses Indicators, in conjunction with Statistics, Positions, and Orders
// - Based on analysis, generates optional Order<Request>
// - Allocates Order<Request>
// - Decides Order<Request> OrderKind
