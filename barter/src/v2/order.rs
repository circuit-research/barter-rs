use barter_integration::model::Side;
use chrono::{DateTime, Utc};
use derive_more::{Constructor, Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Deserialize,
    Serialize,
    Display,
    From,
    Constructor,
)]
pub struct ClientOrderId<T = Uuid>(pub T);

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Deserialize,
    Serialize,
    Display,
    From,
    Constructor,
)]
pub struct OrderId<T = String>(pub T);

#[derive(
    Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize, Constructor,
)]
pub struct Order<InstrumentKey, State> {
    pub instrument: InstrumentKey,
    pub cid: ClientOrderId,
    pub side: Side,
    pub state: State,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, From)]
pub enum OrderState {
    Open(Open),
    Cancelled(Cancelled),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Constructor)]
pub struct RequestOpen {
    pub kind: OrderKind,
    pub time_in_force: TimeInForce,
    pub price: f64,
    pub quantity: f64,
}

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize, Display,
)]
pub enum OrderKind {
    Limit,
}

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize, Display,
)]
pub enum TimeInForce {
    GoodUntilCancelled { post_only: bool },
    GoodUntilEndOfDay,
    FillOrKill,
    ImmediateOrCancel,
}

#[derive(
    Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize, Constructor,
)]
pub struct RequestCancel {
    pub id: OrderId,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct InFlight;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Constructor)]
pub struct Open {
    pub id: OrderId,
    pub time_update: DateTime<Utc>,
    pub price: f64,
    pub quantity: f64,
    pub filled_quantity: f64,
}

impl Open {
    pub fn quantity_remaining(&self) -> f64 {
        self.quantity - self.filled_quantity
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize, Constructor,
)]
pub struct Cancelled {
    pub id: OrderId,
    pub time_exchange: DateTime<Utc>,
}
