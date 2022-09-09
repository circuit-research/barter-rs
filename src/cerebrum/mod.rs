use self::{
    account::AccountUpdater,
    consumer::Consumer,
    event::EventFeed,
    market::MarketUpdater,
    order::{OrderGenerator, Algorithmic, Manual},
    command::Commander,
};


mod consumer;
mod event;
mod account;
mod market;
mod order;
mod command;

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

// pub struct Start;
pub struct Terminated;

// pub struct Engine<State> {
//     feed: EventFeed,
//     cerebrum: Cerebrum<State>
// }

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

    pub fn next(mut self) -> Self {
        // Todo: optimise naming here eg/ engine.next_event(), engine.update_from_market()
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


#[test]
fn it_works() {
    // EventFeed
    let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
    let feed = EventFeed::new(event_rx);

    let mut engine = Engine::new(feed);

    let end = 'trading: loop {
        let x = match engine.next() {
            Engine::Terminated(end) => {
                break end;
            }
            _ => continue
        };
    };
}