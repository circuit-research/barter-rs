use self::{
    account::AccountUpdater,
    consumer::Consumer,
    event::EventFeed,
    market::MarketUpdater,
    order::{OrderGenerator, Algorithmic, Manual},
    command::Commander,
};
use std::collections::VecDeque;

mod consumer;
mod event;
mod account;
mod market;
mod order;
mod command;

// Todo: I could make one receiver for all events...? ie/ MarketFeed + AccountFeed + Command
//  '--> State would change depending on the event type
//  '--> Would still need a queue to clear before we consume next Event
//  '--> Determine what fields go in what state later eg/ event_q only for command?

//  - Add SignalGenerator state for assessing state of Stats & Indicator?

// 1 State Machine w/ Queue
// 1.1 Consume next enum Event (Market, Account, Command)
// 1.1a Market -> Update PositionValue, Statistics, Indicators
// 1.1b Account -> Update Position, Orders, Balances, Statistics
// 1.1c Command -> Add some non-organic event to the Queue to handle before consuming next Event

pub enum CerebrumState {
    // Start(Cerebrum<Start>),
    Consumer(Cerebrum<Consumer>),
    MarketUpdater(Cerebrum<MarketUpdater>),
    OrderGeneratorManual(Cerebrum<OrderGenerator<Manual>>),
    OrderGeneratorAlgorithmic(Cerebrum<OrderGenerator<Algorithmic>>),
    AccountUpdater(Cerebrum<AccountUpdater>),
    Commander(Cerebrum<Commander>),
    End(Cerebrum<End>),
}

pub struct Cerebrum<State> {
    pub feed: EventFeed,
    pub event_tx: (),
    pub event_q: VecDeque<()>,
    pub balances: (),
    pub orders: (),
    pub positions: (),
    pub strategy: (),
    pub state: State,
}

// pub struct Start;
pub struct End;


#[test]
fn it_works() {
    // Construct dependencies

    // EventFeed
    let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
    let feed = EventFeed::new(event_rx);


    // let cerebrum = Cerebrum {
    //     feed,
    //     event_q: Default::default(),
    //     balances: (),
    //     orders: (),
    //     positions: (),
    //     strategy: ()
    // }
}