use super::{
    AccountUpdater, MarketUpdater,
    position::Position,
};
use barter_integration::model::{Exchange, Instrument, Symbol};
use barter_data::model::MarketEvent;
use barter_execution::model::{
    AccountEvent, AccountEventKind, ClientOrderId,
    balance::Balance,
    order::{InFlight, Open, Order}
};
use std::collections::HashMap;

pub struct Account {
    pub exchange: Exchange,
    pub balances: HashMap<Symbol, Balance>,
    pub positions: HashMap<Instrument, Position>,
    pub orders_in_flight: HashMap<ClientOrderId, Order<InFlight>>,
    pub orders_open: HashMap<ClientOrderId, Order<Open>>,
}

impl MarketUpdater for Account {
    fn update_from_market(&mut self, market: &MarketEvent) {
        todo!()
    }
}

impl AccountUpdater for Account {
    fn update_from_account(&mut self, account: &AccountEvent) {
        match &account.kind {
            AccountEventKind::OrdersOpen(open) => {

            }
            AccountEventKind::OrdersNew(new) => {

            }
            AccountEventKind::OrdersCancelled(cancelled) => {

            }
            AccountEventKind::Balance(balance) => {

            }
            AccountEventKind::Balances(balances) => {

            }
            AccountEventKind::Trade(trade) => {

            }
        }
    }
}
