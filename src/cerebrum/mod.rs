use self::{
    event::EventFeed,
    initialise::Initialiser,
    market::MarketUpdater,
    account::AccountUpdater,
    command::Commander,
    consume::Consumer,
    terminate::Terminated,
    order::{Algorithmic, Manual, OrderGenerator},
};
use crate::{
    engine::error::EngineError,
};



mod consume;
mod event;
mod account;
mod market;
mod order;
mod command;
mod terminate;
mod initialise;

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

// Could use something like? Whatever is decided be consistent with casing / verb usage etc.
pub enum EngineNew {
    Initialise(Cerebrum<Initialiser>),
    Consume(Cerebrum<Consumer>),
    UpdateMarket(Cerebrum<MarketUpdater>),
    UpdateAccount(Cerebrum<AccountUpdater>),
    GenerateOrderAlgorithmic(Cerebrum<OrderGenerator<Algorithmic>>),
    GenerateOrderManual(Cerebrum<OrderGenerator<Manual>>),
    ActionCommand(Cerebrum<Commander>),
    TerminateCerebrum<Terminated>),
}


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
    pub event_tx: (),
    pub balances: (),
    pub orders: (),
    pub positions: (),
    pub strategy: (),
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
    pub event_tx: Option<()>,
    pub balances: Option<()>,
    pub orders: Option<()>,
    pub positions: Option<()>,
    pub strategy: Option<()>,
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

    pub fn event_tx(self, value: ()) -> Self {
        Self {
            event_tx: Some(value),
            ..self
        }
    }

    pub fn balances(self, value: ()) -> Self {
        Self {
            balances: Some(value),
            ..self
        }
    }

    pub fn orders(self, value: ()) -> Self {
        Self {
            orders: Some(value),
            ..self
        }
    }

    pub fn positions(self, value: ()) -> Self {
        Self {
            positions: Some(value),
            ..self
        }
    }

    pub fn strategy(self, value: ()) -> Self {
        Self {
            strategy: Some(value),
            ..self
        }
    }

    pub fn build(self) -> Result<Engine, EngineError> {
        Ok(Engine::Initialiser(Cerebrum {
            state: Initialiser,
            feed: self.feed.ok_or(EngineError::BuilderIncomplete("engine_id"))?,
            event_tx: self.event_tx.ok_or(EngineError::BuilderIncomplete("event_tx"))?,
            balances: self.balances.ok_or(EngineError::BuilderIncomplete("balances"))?,
            orders: self.orders.ok_or(EngineError::BuilderIncomplete("orders"))?,
            positions: self.positions.ok_or(EngineError::BuilderIncomplete("positions"))?,
            strategy: self.strategy.ok_or(EngineError::BuilderIncomplete("strategy"))?
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
    use crate::cerebrum::event::{Command, Event};
    use crate::data::live::MarketFeed;
    use crate::data::MarketGenerator;
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

    #[tokio::test]
    async fn it_works() {
        // EventFeed
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let feed = EventFeed::new(event_rx);

        // Spawn MarketFeed on separate thread
        market_feed(event_tx.clone()).await;


        let mut engine = Engine::builder()
            .feed(feed)
            .event_tx(()).balances(()).orders(()).positions(()).strategy(())
            .build()
            .unwrap();

        std::thread::spawn(move || {
            engine.run()
        });

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            event_tx.send(Event::Command(Command::Terminate));
        }).await;


        tokio::time::sleep(Duration::from_secs(5)).await
    }

}

