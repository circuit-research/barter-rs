use std::collections::HashMap;
use barter_data::model::MarketEvent;
use barter_integration::model::{Exchange, Instrument, Symbol};
use uuid::Uuid;
use crate::cerebrum::event::{AccountEventKind, Balance, SymbolBalance};
use super::{
    Cerebrum, consume::Consumer,
    Engine,
    event::AccountEvent,
};

/// AccountUpdater can transition to:
///  a) Consumer
pub struct AccountUpdater;

impl<Strategy> Cerebrum<AccountUpdater, Strategy> {
    pub fn update(mut self, account: AccountEvent) -> Engine<Strategy> {
        // Update Positions, Statistics, Indicators
        match account.kind {
            AccountEventKind::OrderNew => {
                // Todo:
                println!("update_from_account: OrderNew");
            }
            AccountEventKind::OrderCancelled => {
                // Todo:
                println!("update_from_account: OrderCancelled");
            }
            AccountEventKind::Trade => {
                // Todo:
                println!("update_from_account: Trade");
            }
            AccountEventKind::Balance(balance) => {
                println!("update_from_account: Balance");
                self.accounts.update_balance(&account.exchange, &balance);
            }
            AccountEventKind::Balances(balances) => {
                println!("update_from_account: Balances");
                self.accounts.update_balances(&account.exchange, &balances);
            }
            AccountEventKind::ConnectionStatus(status) => {
                // Todo:
                println!("update_from_account: {status:?}");
            }
        };

        Engine::Consumer(Cerebrum::from(self))
    }
}


/// a) AccountUpdater -> Consumer
impl<Strategy> From<Cerebrum<AccountUpdater, Strategy>> for Cerebrum<Consumer, Strategy> {
    fn from(cerebrum: Cerebrum<AccountUpdater, Strategy>) -> Self {
        Self {
            state: Consumer,
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            audit_tx: cerebrum.audit_tx,
        }
    }
}


pub struct Accounts(pub HashMap<Exchange, Account>);

pub struct Account {
    pub balances: HashMap<Symbol, Balance>,
    pub positions: HashMap<Instrument, Position>,
    pub orders: Orders,
}

impl Accounts {
    pub fn account(&mut self, exchange: &Exchange) -> &mut Account {
        self.0
            .get_mut(exchange)
            .expect("cannot retrieve Account for unexpected Exchange")
    }

    pub fn update_balance(&mut self, exchange: &Exchange, balance: &SymbolBalance) {
         self.account(exchange)
            .balances
            .get_mut(&balance.symbol)
            .and_then(|account_balance| {
                account_balance.total = balance.balance.total;
                account_balance.available = balance.balance.available;

                println!("Symbol: {}, New Balance: {:?}", balance.symbol, account_balance);

                Some(account_balance)
            })
            .expect("cannot update Balance for unexpected Symbol");
    }

    pub fn update_balances(&mut self, exchange: &Exchange, balances: &Vec<SymbolBalance>) {
        balances
            .into_iter()
            .for_each(|balance| self.update_balance(exchange, balance))
    }

    pub fn update_positions(&mut self, market: &MarketEvent) {
        // Todo: Update relevant Positions
    }
}

pub struct Position;
pub struct Orders {
    pub in_flight: HashMap<ClientOrderId, ()>,
    pub open: HashMap<ClientOrderId, ()>,
}
pub struct ClientOrderId(pub Uuid);
