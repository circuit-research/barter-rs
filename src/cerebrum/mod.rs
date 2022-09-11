use self::{
    account::{Accounts, AccountUpdater},
    command::Commander,
    consume::Consumer,
    event::{AccountEvent, EventFeed},
    exchange::ExchangeRequest,
    initialise::Initialiser,
    market::MarketUpdater,
    order::{Algorithmic, Manual, OrderGenerator},
    terminate::Terminated,
    strategy::IndicatorUpdater,
};
use crate::engine::error::EngineError;
use barter_data::model::MarketEvent;
use tokio::sync::mpsc;

pub mod consume;
pub mod event;
pub mod account;
pub mod market;
pub mod order;
pub mod command;
pub mod terminate;
pub mod initialise;
pub mod exchange;
pub mod strategy;
mod simulated;

// Todo:
//  - Could have a thread for each exchange that processes MarketEvents -> Indicators/Statistics
//   '--> Those Indicators/Statistics are then the input into the system (if there are performance issues)
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
//   '--> Change update_balance() .expect() for error! like open order logic
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
//  - Should I have the concept & tracking of orders_in_flight_cancel? Along with associated State InFlightCancel?
//  - Account should probably have an Exchange -> perhaps Accounts<'a>(HashMap<&'a Exchange, Account>)
//  - Add states to transition to when we go unhealthy via ConnectionStatus eg/ CancelOnly, Offline etc.
//   '--> Can we add the MarketFeed health into this also? Want the MarketFeed to send ConnectionStatus / health
//        to EventFeed also, which can alter the EngineState
//  - Perhaps the EventFeed should just be a std::mpsc::unbounded? or crossbeam etc.
//  - Engine probably contains handles to the ExchangePortal, etc. Builder could do all of init...
//  - Would the idea of an ExchangeId::Simulated(u8) be satisfactory rather than Exchange?
//  - Make as much stuff reference as possible, eg/ Accounts could use reference Accounts<'a>(HashMap<&'a Symbol...)
//  - Make ExchangePortal generic so an Engine can be select with a higher performance portal for
//    a single Exchange only :) Same goes for all other multi-exchange functionality...

pub struct Components<Strategy> {
    feed: EventFeed,
    accounts: Accounts,
    exchange_tx: mpsc::UnboundedSender<ExchangeRequest>,
    strategy: Strategy,
    audit_tx: ()
}

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
    pub request_tx: mpsc::UnboundedSender<ExchangeRequest>,
    pub strategy: Strategy,
    pub audit_tx: (),
}

impl<Strategy> Engine<Strategy>
    where
        Strategy: IndicatorUpdater + strategy::OrderGenerator,
{
    pub fn new(components: Components<Strategy>) -> Self {
        Self::Initialiser(Cerebrum {
            state: Initialiser,
            feed: components.feed,
            accounts: components.accounts,
            request_tx: components.exchange_tx,
            strategy: components.strategy,
            audit_tx: components.audit_tx
        })
    }

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
                cerebrum.generate_order_requests()
            }
            Self::OrderGeneratorManual((cerebrum, meta)) => {
                cerebrum.generate_order_requests_manual(meta)
            },
            Self::AccountUpdater((cerebrum, account)) => {
                cerebrum.update(account)
            }
            Self::Commander(cerebrum) => {
                cerebrum.execute_manual_command()
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
    pub exchange_tx: Option<mpsc::UnboundedSender<ExchangeRequest>>,
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

    pub fn exchange_tx(self, value: mpsc::UnboundedSender<ExchangeRequest>) -> Self {
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
            request_tx: self.exchange_tx.ok_or(EngineError::BuilderIncomplete("exchange_tx"))?,
            strategy: self.strategy.ok_or(EngineError::BuilderIncomplete("strategy"))?,
            audit_tx: self.audit_tx.ok_or(EngineError::BuilderIncomplete("audit_tx"))?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        time::Duration,
    };
    use std::ops::Add;
    use crate::{
        cerebrum::{
            account::{Account, Accounts, Position},
            Engine,
            event::{AccountEvent, AccountEventKind, Balance, Command, Event, EventFeed}, exchange::ExchangeRequest,
            order::{Order, RequestCancel, RequestOpen},
            strategy,
            strategy::IndicatorUpdater,

        },
        data::{Feed, live::MarketFeed, MarketGenerator}
    };
    use barter_data::{
        ExchangeId,
        model::{DataKind, MarketEvent, SubKind, Subscription}
    };
    use barter_integration::model::{Exchange, Instrument, InstrumentKind, Market, Symbol};
    use chrono::Utc;
    use tokio::sync::mpsc;
    use tokio::sync::mpsc::error::TryRecvError;
    use barter_execution::model::ConnectionStatus;
    use crate::cerebrum::event::SymbolBalance;

    struct StrategyExample {
        rsi: ta::indicators::RelativeStrengthIndex,
    }

    impl IndicatorUpdater for StrategyExample {
        fn update_indicators(&mut self, market: &MarketEvent) {
            match &market.kind {
                DataKind::Trade(trade) => trade.price,
                DataKind::Candle(candle) => candle.close,
            };
        }
    }

    impl strategy::OrderGenerator for StrategyExample {
        fn generate_cancels(&self) -> Option<Vec<Order<RequestCancel>>> {
            None
        }

        fn generate_orders(&self) -> Option<Vec<Order<RequestOpen>>> {
            None
        }
    }

    // Notes:
    // - Hard-coded to use one Exchange, Ftx
    #[tokio::test]
    async fn it_works() {
        // Initialise structured JSON subscriber
        init_logging();

        // Duration to run before Termination
        let terminate = Duration::from_secs(6000);

        // Load Subscriptions
        let subscriptions = load_subscriptions();

        // Central EventFeed: will receive Event::Market, Event::Account & Event::Command
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let feed = EventFeed::new(event_rx);

        // ExchangeCommand Transmitter
        let (exchange_tx, exchange_rx) = mpsc::unbounded_channel();

        // Event Audit Transmitter: Stubbed For Now
        // let (audit_tx, audit_rx) = mpsc::unbounded_channel();
        let audit_tx = ();

        // EventFeed Component: MarketFeed:
        init_market_feed(event_tx.clone(), &subscriptions).await;

        // EventFeed Component: AccountFeed:
        init_account_feed(event_tx.clone(), exchange_rx);

        // EventFeed Component: CommandFeed
        init_command_feed(event_tx, terminate);

        // Accounts(HashMap<Exchange, Account>):
        let accounts = init_accounts(
            Exchange::from(ExchangeId::Ftx), subscriptions
        );

        // StrategyExample
        let strategy = StrategyExample {
            rsi: ta::indicators::RelativeStrengthIndex::new(14).unwrap()
        };

        // Build Engine
        let engine = Engine::builder()
            .feed(feed) // Todo: Should builder set this up?
            .accounts(accounts) // Todo: Should builder set this up?
            .exchange_tx(exchange_tx)
            .strategy(strategy)
            .audit_tx(audit_tx)
            .build()
            .expect("failed to build Engine");

        // Run Engine
        std::thread::spawn(move || {
            engine.run()
        });

        // tokio::task::spawn(async move {
        //     engine.run()
        // }).await.unwrap();

        tokio::time::sleep(terminate.add(Duration::from_secs(1))).await
    }

    fn load_subscriptions() -> Vec<Subscription> {
        vec![
            Subscription::new(
                ExchangeId::Ftx,
                ("btc", "usdt", InstrumentKind::FuturePerpetual),
                SubKind::Trade
            ),
            Subscription::new(
                ExchangeId::Ftx,
                ("eth", "usdt", InstrumentKind::FuturePerpetual),
                SubKind::Trade
            ),
            Subscription::new(
                ExchangeId::Ftx,
                ("xrp", "usdt", InstrumentKind::FuturePerpetual),
                SubKind::Trade
            ),
        ]
    }

    async fn init_market_feed(event_tx: mpsc::UnboundedSender<Event>, subscriptions: &Vec<Subscription>) {
        let mut market_rx = MarketFeed::init(subscriptions.clone())
            .await
            .expect("failed to initialise MarketFeed")
            .market_rx;

        std::thread::spawn(move || {
            loop {
                match market_rx.try_recv() {
                    Ok(market) => {
                        event_tx
                            .send(Event::Market(market))
                            .expect("failed to send MarketEvent to EventFeed")
                    },
                    Err(mpsc::error::TryRecvError::Empty) => {
                        // panic!("MarketFeed empty")
                        continue
                    },
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        panic!("MarketFeed failed")
                    },
                }
            }
        });
    }

    // Todo:
//  - Will change when we setup the ExchangeClients properly, likely needs Vec<Instrument>
    fn init_account_feed(event_tx: mpsc::UnboundedSender<Event>, mut exchange_rx: mpsc::UnboundedReceiver<ExchangeRequest>) {
        tokio::task::spawn(async move {
            while let Some(request) = exchange_rx.recv().await {
                match request {
                    ExchangeRequest::FetchOpenOrders(_)=> {
                        // Todo:
                    }
                    ExchangeRequest::FetchBalances(_) => {
                        // Todo:
                    }
                    ExchangeRequest::OpenOrders(_) => {
                        // Todo:
                    }
                    ExchangeRequest::CancelOrders(_) => {
                        // Todo:
                    }
                    ExchangeRequest::CancelOrdersAll(_) => {
                        // Todo:
                    }
                }
            }
        });
    }

    fn init_command_feed(event_tx: mpsc::UnboundedSender<Event>, terminate: Duration) {
        std::thread::spawn(move || {
            std::thread::sleep(terminate);
            event_tx
                .send(Event::Command(Command::Terminate))
                .unwrap()
        });
    }

    fn init_accounts(exchange: Exchange, subscriptions: Vec<Subscription>) -> Accounts {
        let instruments = subscriptions
            .into_iter()
            .map(|subscription| subscription.instrument)
            .collect();

        let mut accounts = HashMap::new();
        accounts.insert(exchange, init_account(instruments));
        Accounts(accounts)
    }

    fn init_account(instruments: Vec<Instrument>) -> Account {
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

