use std::time::Duration;
use terminate::Terminated;
use crate::cerebrum::event::{Command, Event};
use self::{
    account::AccountUpdater,
    command::Commander,
    consume::Consumer,
    event::EventFeed,
    market::MarketUpdater,
    order::{Algorithmic, Manual, OrderGenerator},
};


mod consume;
mod event;
mod account;
mod market;
mod order;
mod command;
mod terminate;

// Todo:
//  - Do I need an event_q?
//  - Determine what fields go in what state later
//  - Rename CerebrumState as Engine?

pub enum Engine {
    // Start(Cerebrum<Start>),
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
    pub fn new(feed: EventFeed) -> Self {
        Self::Consumer(Cerebrum {
            state: Consumer,
            feed,
            event_tx: (),
            balances: (),
            orders: (),
            positions: (),
            strategy: (),
        })
    }

    pub fn run(mut self) {
        'trading: loop {
            // Transition to the next trading state
            self = self.next();

            // Engine terminated

            if let Engine::Terminated(_) = self {
                // Todo: Print trading session results & persist
                break
            }
        }
    }

    pub fn next(mut self) -> Self {
        match self {
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


#[tokio::test]
async fn it_works() {
    // EventFeed
    let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
    let feed = EventFeed::new(event_rx);
    let mut engine = Engine::new(feed);

    std::thread::spawn(move || {
        engine.run()
    });

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        event_tx.send(Event::Command(Command::Terminate));
    }).await;


    tokio::time::sleep(Duration::from_secs(5)).await
}