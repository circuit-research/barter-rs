use barter_integration::model::{Exchange, Instrument, Symbol};
use barter_data::model::MarketEvent;
use barter_execution::{
    model::{AccountEvent, balance::Balance}
};
use std::collections::HashMap;
use barter_execution::model::ClientOrderId;
use barter_execution::model::order::{InFlight, Open, Order};

pub struct Position;

pub trait MarketUpdater {
    fn update_from_market(&mut self, market: MarketEvent);
}

pub trait AccountUpdater {
    fn update_from_account(&mut self, account: AccountEvent);
}

pub struct Account {
    pub exchange: Exchange,
    pub balances: HashMap<Symbol, Balance>,
    pub positions: HashMap<Instrument, Position>,
    pub orders_in_flight: HashMap<ClientOrderId, Order<InFlight>>,
    pub orders_open: HashMap<ClientOrderId, Order<Open>>,
}

impl MarketUpdater for Account {
    fn update_from_market(&mut self, market: MarketEvent) {

    }
}

impl AccountUpdater for Account {
    fn update_from_account(&mut self, account: AccountEvent) {

    }
}

pub trait Updater {
    type Components;
    type Event;

    fn new(components: Self::Components) -> Self;
    fn update(&mut self, event: Self::Event);
}

impl Updater for Account {
    type Components = ();
    type Event = MarketEvent;

    fn new(components: Self::Components) -> Self {
        todo!()
    }

    fn update(&mut self, event: Self::Event) {
        todo!()
    }
}