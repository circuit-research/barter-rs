use self::{
    event::EventFeed,
    initialise::Initialiser,
    market::MarketUpdater,
    account::AccountUpdater,
    command::Commander,
    consume::Consumer,
    exchange::ExchangeCommand,
    terminate::Terminated,
    order::{Algorithmic, Manual, OrderGenerator},
};
use crate::{
    engine::error::EngineError,
};
use tokio::sync::mpsc;


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

pub enum Engine {
    Initialiser(Cerebrum<Initialiser>),
    Consumer(Cerebrum<Consumer>),
    MarketUpdater(Cerebrum<MarketUpdater>),
    OrderGeneratorAlgorithmic(Cerebrum<OrderGenerator<Algorithmic>>),
    OrderGeneratorManual(Cerebrum<OrderGenerator<Manual>>),
    AccountUpdater(Cerebrum<AccountUpdater>),
    Commander(Cerebrum<Commander>),
    Terminated(Cerebrum<Terminated>),
}

pub struct Cerebrum<State> {
    pub state: State,
    pub feed: EventFeed,
    pub accounts: Accounts,
    pub exchange_tx: mpsc::UnboundedSender<ExchangeCommand>,
    pub strategy: (),
    pub event_tx: (),
}

pub struct Accounts {
    balances: (),
    positions: (),
    orders: (),
}

impl Engine {
    pub fn builder() -> EngineBuilder {
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
            Self::MarketUpdater(engine) => {
                engine.update_from_market_event()
            },
            Self::OrderGeneratorAlgorithmic(engine) => {
                engine.generate_order()
            }
            Self::OrderGeneratorManual(engine) => {
                engine.generate_order_manual()
            },
            Self::AccountUpdater(engine) => {
                engine.update_from_account_event()
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
pub struct EngineBuilder {
    pub feed: Option<EventFeed>,
    pub accounts: Option<Accounts>,
    pub exchange_tx: Option<mpsc::UnboundedSender<ExchangeCommand>>,
    pub strategy: Option<()>,
    pub event_tx: Option<()>,
}

impl EngineBuilder {
    fn new() -> Self {
        Self::default()
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

    pub fn strategy(self, value: ()) -> Self {
        Self {
            strategy: Some(value),
            ..self
        }
    }

    pub fn event_tx(self, value: ()) -> Self {
        Self {
            event_tx: Some(value),
            ..self
        }
    }

    pub fn build(self) -> Result<Engine, EngineError> {
        Ok(Engine::Initialiser(Cerebrum {
            state: Initialiser,
            feed: self.feed.ok_or(EngineError::BuilderIncomplete("engine_id"))?,
            accounts: self.accounts.ok_or(EngineError::BuilderIncomplete("account"))?,
            exchange_tx: self.exchange_tx.ok_or(EngineError::BuilderIncomplete("exchange_tx"))?,
            strategy: self.strategy.ok_or(EngineError::BuilderIncomplete("strategy"))?,
            event_tx: self.event_tx.ok_or(EngineError::BuilderIncomplete("event_tx"))?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use barter_data::ExchangeId;
    use barter_data::model::SubKind;
    use barter_integration::model::InstrumentKind;
    use tokio::sync::mpsc;
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
        let accounts = Accounts { balances: (), positions: (), orders: ()};

        // ExchangeCommandTx
        let (exchange_tx, exchange_rx) = mpsc::unbounded_channel();

        // Spawn STUBBED AccountFEed on separate thread
        account_feed(event_tx.clone()).await;

        let mut engine = Engine::builder()
            .feed(feed)
            .event_tx(())
            .accounts(accounts)
            .exchange_tx(exchange_tx)
            .strategy(())
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

