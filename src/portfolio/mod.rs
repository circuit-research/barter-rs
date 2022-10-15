use crate::{
    event::EventFeed,
    engine::error::EngineError,
    execution::ExecutionRequest,
};
use barter_integration::model::{Exchange, Instrument};
use barter_data::model::MarketEvent;
use barter_execution::model::AccountEvent;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub mod account;
pub mod position;

pub trait Initialiser {
    type Output;

    fn init(
        instruments: HashMap<Exchange, Vec<Instrument>>,
        execution_tx: &mpsc::UnboundedSender<ExecutionRequest>,
        feed: &mut EventFeed,
    ) -> Result<Self::Output, EngineError>;
}

pub trait MarketUpdater {
    fn update_from_market(&mut self, market: &MarketEvent);
}

pub trait AccountUpdater {
    fn update_from_account(&mut self, account: &AccountEvent);
}

