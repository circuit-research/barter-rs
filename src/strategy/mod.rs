use barter_execution::model::order::{Order, RequestCancel, RequestOpen};

pub trait OrderGenerator {
    fn generate_cancels(&self) -> Option<Vec<Order<RequestCancel>>>;
    fn generate_orders(&self) -> Option<Vec<Order<RequestOpen>>>;
}