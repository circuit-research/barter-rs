use self::{
    account::{Accounts, AccountUpdater},
    command::Commander,
    consume::Consumer,
    event::{AccountEvent, EventFeed},
    exchange::ExchangeCommand,
    initialise::Initialiser,
    market::MarketUpdater,
    order::{Algorithmic, Manual, OrderGenerator},
    terminate::Terminated,
};
use crate::engine::error::EngineError;
use barter_data::model::MarketEvent;
use tokio::sync::mpsc;
use strategy::IndicatorUpdater;

mod consume;
mod event;
mod account;
mod market;
mod order;
mod command;
mod terminate;
mod initialise;
mod exchange;
mod strategy;

// Todo:
//  - Derive as eagerly as possible
//  - Add metric_tx stub?
//  - Determine what fields go in what state later
//  - Will need some startup States to go from New -> Initialised
//   '--> logic to hit the ExchangeClient to get balances, orders, positions.
//     '--> Start off with one ExchangeClient before adding many exchanges...
//     '--> exchange_tx / execution_tx / account_tx (or similar) to send Requests to exchange
//  - Make input & output feed / tx / rx names more distinct eg/ InputEventFeed, or InputFeed...
//     ... output_tx / audit_tx / state_tx etc
//  - EngineState naming to be decided, but be consistent with casing / verb usage etc.
//  - Consumer state can likely transition to Initialiser while we wait for responses from exchange?
//  - Feed needs some work to be more like MarketFeed w/ Feed struct? etc.
//  - Account or Portfolio? Change name of AccountUpdater and AccountEvent if we do change, etc.
//  - Should the Strategy have control over 'Account'?
//  - Engine will also have control of spawning the execution clients, presumably...?
//   '--> Perhaps Engine will need to be a struct and enum Engine -> CerebrumState/TradingState/Trader
//   '--> Perhaps the builder could do the Init of Cerebrum in a blocking way
//  - Strategy could use Associated types?
//  - Do I want Accounts to have Positions? Also MarketUpdater would call self.accounts.update_positions()
//   '--> is it relevant now we hold every Instrument?
//   '--> How to track PnL? What is a Position?
//  - AccountUpdater no longer updates Positions & Statistics, but just Indicators -> this may change back?
//   '--> If we only update Indicators, would this become SignalUpdater?
//  - Send Events to the audit_tx eg/ update_from_market { event_tx.send(market).unwrap() }
//  - Is it valid to have a SymbolBalance, or do we need the idea of SymbolInstrumentKindBalance?
//  - Update Balances can be more efficient since we know what Markets we trade at the start
//   '--> Change update_balance() .expect() for error! like open orde rlogic
//   '--> Can ignore anything which contains_key() returns None etc
//  - More efficient to use Accounts(Vec<(Exchange, Account)) (or have Exchange inside Account?
//    '--> Benchmark, but probably faster for since people won't use many exchanges at once?
//  - Work out how to do fees for trade, and add Liquidity field?
//  - Impl display for MarketEvent, AccountEvent, Command
//  - Could make Account generic to give it functionality to generate appropriate Cid?
//  - Ensure I am happy with log levels after dev. eg/ update_order_from_cancel logs at worn if we
//    cancel in flight order
//  - self.accounts.update_orders_from_open(&order); is taking ref & cloning - only makes sense if
//    we are using audit_tx... double check this later

pub enum Engine<Strategy> {
    Initialiser(Cerebrum<Initialiser, Strategy>),
    Consumer(Cerebrum<Consumer, Strategy>),
    MarketUpdater((Cerebrum<MarketUpdater, Strategy>, MarketEvent)),
    OrderGeneratorAlgorithmic(Cerebrum<OrderGenerator<Algorithmic>, Strategy>),
    OrderGeneratorManual((Cerebrum<OrderGenerator<Manual>, Strategy>, ())),
    AccountUpdater((Cerebrum<AccountUpdater, Strategy>, AccountEvent)),
    Commander(Cerebrum<Commander, Strategy>),
    Terminated(Cerebrum<Terminated, Strategy>),
}

pub struct Cerebrum<State, Strategy> {
    pub state: State,
    pub feed: EventFeed,
    pub accounts: Accounts,
    pub exchange_tx: mpsc::UnboundedSender<ExchangeCommand>,
    pub strategy: Strategy,
    pub audit_tx: (),
}

impl<Strategy> Engine<Strategy>
where
    Strategy: IndicatorUpdater,
{
    pub fn builder() -> EngineBuilder<Strategy> {
        EngineBuilder::new()
    }

    pub fn run(mut self) {
        'trading: loop {
            // Transition to the next trading state
            self = self.next();

            if let Engine::Terminated(_) = self {
                // Todo: Print trading session results & persist
                break 'trading
            }
        }
    }

    pub fn next(mut self) -> Self {
        match self {
            Self::Initialiser(cerebrum) => {
                cerebrum.init()
            }
            Self::Consumer(cerebrum) => {
                cerebrum.next_event()
            },
            Self::MarketUpdater((cerebrum, market)) => {
                cerebrum.update(market)
            },
            Self::OrderGeneratorAlgorithmic(cerebrum) => {
                cerebrum.generate_order()
            }
            Self::OrderGeneratorManual((cerebrum, meta)) => {
                cerebrum.generate_order_manual(meta)
            },
            Self::AccountUpdater((cerebrum, account)) => {
                cerebrum.update(account)
            }
            Self::Commander(cerebrum) => {
                cerebrum.action_manual_command()
            }
            Self::Terminated(cerebrum) => {
                Self::Terminated(cerebrum)
            }
        }
    }
}

/// Builder to construct [`Engine`] instances.
#[derive(Default)]
pub struct EngineBuilder<Strategy> {
    pub feed: Option<EventFeed>,
    pub accounts: Option<Accounts>,
    pub exchange_tx: Option<mpsc::UnboundedSender<ExchangeCommand>>,
    pub strategy: Option<Strategy>,
    pub audit_tx: Option<()>,
}

impl<Strategy> EngineBuilder<Strategy> {
    fn new() -> Self {
        Self {
            feed: None,
            accounts: None,
            exchange_tx: None,
            strategy: None,
            audit_tx: None
        }
    }

    pub fn feed(self, value: EventFeed) -> Self {
        Self {
            feed: Some(value),
            ..self
        }
    }

    pub fn accounts(self, value: Accounts) -> Self {
        Self {
            accounts: Some(value),
            ..self
        }
    }

    pub fn exchange_tx(self, value: mpsc::UnboundedSender<ExchangeCommand>) -> Self {
        Self {
            exchange_tx: Some(value),
            ..self
        }
    }

    pub fn strategy(self, value: Strategy) -> Self {
        Self {
            strategy: Some(value),
            ..self
        }
    }

    pub fn audit_tx(self, value: ()) -> Self {
        Self {
            audit_tx: Some(value),
            ..self
        }
    }

    pub fn build(self) -> Result<Engine<Strategy>, EngineError> {
        Ok(Engine::Initialiser(Cerebrum {
            state: Initialiser,
            feed: self.feed.ok_or(EngineError::BuilderIncomplete("engine_id"))?,
            accounts: self.accounts.ok_or(EngineError::BuilderIncomplete("account"))?,
            exchange_tx: self.exchange_tx.ok_or(EngineError::BuilderIncomplete("exchange_tx"))?,
            strategy: self.strategy.ok_or(EngineError::BuilderIncomplete("strategy"))?,
            audit_tx: self.audit_tx.ok_or(EngineError::BuilderIncomplete("audit_tx"))?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;
    use barter_data::ExchangeId;
    use barter_data::model::{MarketEvent, SubKind};
    use barter_integration::model::{Exchange, Instrument, InstrumentKind, Market, Symbol};
    use tokio::sync::mpsc;
    use crate::cerebrum::account::{Account, Position};
    use crate::cerebrum::event::{AccountEvent, AccountEventKind, Balance, Command, Event, SymbolBalance};
    use crate::data::live::MarketFeed;
    use super::*;

    async fn market_feed(markets: Vec<Market>, event_tx: mpsc::UnboundedSender<Event>) {
        // MarketFeed
        // Todo:
        //  - Does this need to be changed for ergonomics to produce an EventFeed rather than MarketFeed?
        //  - We want to ensure that heavy MarketFeed processing is occurring on it's own thread (not task since it's busy loop)

        let mut market = MarketFeed::init([
            (ExchangeId::Ftx, "btc", "usdt", InstrumentKind::FuturePerpetual, SubKind::Trade),
            (ExchangeId::Ftx, "eth", "usdt", InstrumentKind::FuturePerpetual, SubKind::Trade),
        ]).await.unwrap();

        std::thread::spawn(move || {
            loop {
                match market.market_rx.try_recv() {
                    Ok(market) => {
                        event_tx.send(Event::Market(market));
                    },
                    Err(mpsc::error::TryRecvError::Empty) => {
                        continue
                    },
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        break
                    },
                }
            }
        });
    }

    async fn account_feed(event_tx: mpsc::UnboundedSender<Event>) {
        std::thread::spawn(move || {
            loop {
                event_tx.send(Event::Account(account_event(AccountEventKind::Balance(
                    // Same as at start, but test single Balance AccountEvent
                    SymbolBalance {
                        symbol: Symbol::from("usdt"),
                        balance: Balance { total: 1000.0, available: 1000.0 }
                    }
                ))));

                std::thread::sleep(Duration::from_secs(2));

                event_tx.send(Event::Account(account_event(AccountEventKind::Balances(
                    vec![
                        // Increased btc by 500 since start
                        SymbolBalance {
                            symbol: Symbol::from("btc"),
                            balance: Balance { total: 1500.0, available: 1500.0 }
                        },
                        // Reduced usdt by 500 since start
                        SymbolBalance {
                            symbol: Symbol::from("usdt"),
                            balance: Balance { total: 500.0, available: 500.0 }
                        }
                    ]
                ))));
            };
        });
    }

    fn account_event(kind: AccountEventKind) -> AccountEvent {
        AccountEvent {
            exchange_time: Default::default(),
            received_time: Default::default(),
            exchange: Exchange::from(ExchangeId::Ftx),
            kind
        }
    }

    fn account(instruments: Vec<Instrument>) -> Account {
        let positions = instruments
            .iter()
            .cloned()
            .map(|instrument| (instrument, Position))
            .collect();

        let mut balances = instruments
            .into_iter()
            .map(|instrument| [instrument.base, instrument.quote])
            .flatten()
            // Todo: Later we will init Balances during Init, so this would be (0.0, 0.0) until exchange update
            .map(|symbol| (symbol, Balance { total: 1000.0, available: 1000.0 }))
            .collect();

        Account {
            balances,
            positions,
            orders_in_flight: HashMap::new(),
            orders_open: HashMap::new()
        }
    }

    #[tokio::test]
    async fn it_works() {
        init_logging();

        // Markets
        let exchange = Exchange::from(ExchangeId::Ftx);
        let markets = vec![
            Market::new(exchange.clone(), ("btc", "usdt", InstrumentKind::FuturePerpetual)),
            Market::new(exchange.clone(), ("eth", "usdt", InstrumentKind::FuturePerpetual)),
        ];

        // EventFeed
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let feed = EventFeed::new(event_rx);

        // Spawn MarketFeed on separate thread
        market_feed(markets.clone(), event_tx.clone()).await;

        // Accounts
        let instruments = markets
            .into_iter()
            .map(|market| {
                let Market { exchange, instrument } = market;
                instrument
            })
            .collect();

        let mut accounts = HashMap::new();
        accounts.insert(exchange.clone(), account(instruments));

        // ExchangeCommandTx
        let (exchange_tx, exchange_rx) = mpsc::unbounded_channel();

        // Spawn STUBBED AccountFEed on separate thread
        account_feed(event_tx.clone()).await;

        // Strategy
        struct StubbedStrategy;
        let strategy = StubbedStrategy;

        impl IndicatorUpdater for StubbedStrategy {
            fn update_indicators(&mut self, market: &MarketEvent) {
            }
        }

        let mut engine = Engine::builder()
            .feed(feed)
            .audit_tx(())
            .accounts(Accounts(accounts))
            .exchange_tx(exchange_tx)
            .strategy(StubbedStrategy)
            .build()
            .unwrap();

        std::thread::spawn(move || {
            engine.run()
        });

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(30)).await;
            event_tx.send(Event::Command(Command::Terminate));
        }).await;


        tokio::time::sleep(Duration::from_secs(32)).await
    }

    /// Initialise a `Subscriber` for `Tracing` Json logs and install it as the global default.
    fn init_logging() {
        tracing_subscriber::fmt()
            // Filter messages based on the `RUST_LOG` environment variable
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            // Disable colours on release builds
            .with_ansi(cfg!(debug_assertions))
            // Enable Json formatting
            .json()
            // Install this Tracing subscriber as global default
            .init()
    }

}

