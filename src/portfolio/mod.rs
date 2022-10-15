use barter_integration::model::{Exchange, Instrument, Symbol};
use barter_data::model::MarketEvent;
use barter_execution::{
    model::{AccountEvent, balance::Balance}
};
use std::collections::HashMap;
use barter_execution::model::ClientOrderId;
use barter_execution::model::order::{InFlight, Open, Order};
use tokio::sync::mpsc;
use crate::engine::error::EngineError;
use crate::event::EventFeed;
use crate::execution::ExecutionRequest;

pub struct Position;

pub trait Initialiser {
    type Output;

    fn init(
        instruments: HashMap<Exchange, Vec<Instrument>>,
        execution_tx: &mpsc::UnboundedSender<ExecutionRequest>,
        feed: &mut EventFeed,
    ) -> Result<Self::Output, EngineError>;
}

pub trait MarketUpdater {
    fn update(&mut self, market: &MarketEvent);
}

pub trait AccountUpdater {
    fn update(&mut self, account: &AccountEvent);
}

pub struct Account {
    pub exchange: Exchange,
    pub balances: HashMap<Symbol, Balance>,
    pub positions: HashMap<Instrument, Position>,
    pub orders_in_flight: HashMap<ClientOrderId, Order<InFlight>>,
    pub orders_open: HashMap<ClientOrderId, Order<Open>>,
}

impl MarketUpdater for Account {
    fn update(&mut self, market: &MarketEvent) {

    }
}

impl AccountUpdater for Account {
    fn update(&mut self, account: &AccountEvent) {

    }
}

pub trait Updater<Event> {
    fn update(&mut self, event: Event);
}

