use std::collections::HashMap;
use barter_data::model::MarketEvent;
use barter_integration::model::Market;
use self::{
    account::AccountUpdater,
    command::Commander,
    consume::Consumer,
    event::EventFeed,
    exchange::ExchangeCommand,
    initialise::Initialiser,
    market::MarketUpdater,
    order::{Algorithmic, Manual, OrderGenerator},
    terminate::Terminated,
};
use crate::engine::error::EngineError;
use tokio::sync::mpsc;
use uuid::Uuid;
use account::Accounts;
use crate::cerebrum::market::IndicatorUpdater;


mod consume;
mod event;
mod account;
mod market;
mod order;
mod command;
mod terminate;
mod initialise;
mod exchange;

// Todo:
//  - Derive as eagerly as possible
//  - Do I need an event_q?
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
//  - Will we add a new
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
//  - Rather than tuple, could have eg/ MarketUpdater { Cerebrum, MarketEvent } where Cerebrum has no State
//   '--> States are defined by the parent structure housing the Cerebrum... might be nicer?

pub enum Engine<Strategy> {
    Initialiser(Cerebrum<Initialiser, Strategy>),
    Consumer(Cerebrum<Consumer, Strategy>),
    MarketUpdater((Cerebrum<MarketUpdater, Strategy>, MarketEvent)),
    OrderGeneratorAlgorithmic(Cerebrum<OrderGenerator<Algorithmic>, Strategy>),
    OrderGeneratorManual(Cerebrum<OrderGenerator<Manual>, Strategy>),
    AccountUpdater(Cerebrum<AccountUpdater, Strategy>),
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
            Self::Initialiser(engine) => {
                engine.init()
            }
            Self::Consumer(engine) => {
                engine.next_event()
            },
            Self::MarketUpdater((engine, market)) => {
                engine.update_from_market(market)
            },
            Self::OrderGeneratorAlgorithmic(engine) => {
                engine.generate_order()
            }
            Self::OrderGeneratorManual(engine) => {
                engine.generate_order_manual()
            },
            Self::AccountUpdater(engine) => {
                engine.update_from_account()
            }
            Self::Commander(engine) => {
                engine.action_manual_command()
            }
            Self::Terminated(engine) => {
                Self::Terminated(engine)
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
    use std::time::Duration;
    use barter_data::ExchangeId;
    use barter_data::model::{MarketEvent, SubKind};
    use barter_integration::model::InstrumentKind;
    use tokio::sync::mpsc;
    use crate::cerebrum::account::Orders;
    use crate::cerebrum::event::{AccountEvent, Command, Event};
    use crate::data::live::MarketFeed;
    use super::*;

    async fn market_feed(event_tx: mpsc::UnboundedSender<Event>) {
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
                event_tx.send(Event::Account(AccountEvent::Balances));
                std::thread::sleep(Duration::from_secs(2));
                event_tx.send(Event::Account(AccountEvent::Trade));
            };
        });
    }

    #[tokio::test]
    async fn it_works() {
        // EventFeed
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let feed = EventFeed::new(event_rx);

        // Spawn MarketFeed on separate thread
        market_feed(event_tx.clone()).await;

        // Accounts
        let accounts = Accounts {
            balances: HashMap::new(),
            positions: HashMap::new(),
            orders: Orders { in_flight: HashMap::new(), open: HashMap::new() }
        };

        // ExchangeCommandTx
        let (exchange_tx, exchange_rx) = mpsc::unbounded_channel();

        // Spawn STUBBED AccountFEed on separate thread
        account_feed(event_tx.clone()).await;

        // Strategy
        struct StubbedStrategy;
        let strategy = StubbedStrategy;

        impl IndicatorUpdater for StubbedStrategy {
            fn update_indicators(&mut self, market: &MarketEvent) {
                println!("update indicators from market: {market:?}");
            }
        }

        let mut engine = Engine::builder()
            .feed(feed)
            .audit_tx(())
            .accounts(accounts)
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

}

