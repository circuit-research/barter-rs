use self::{
    event::{EventFeed, Event, AccountEvent, Command},
    consumer::Consumer,
    market::MarketUpdater,
    order::OrderGenerator,
    account::AccountUpdater,
};
use crate::data::Feed;
use barter_data::model::MarketEvent;
use std::collections::VecDeque;
use tokio::sync::mpsc;

mod consumer;
mod event;
mod account;
mod market;
mod order;

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
    OrderGenerator(Cerebrum<OrderGenerator>),
    AccountUpdater(Cerebrum<AccountUpdater>),
    Commander(Cerebrum<Commander>),
    // End(Cerebrum<End>),
}

pub struct Cerebrum<State> {
    pub feed: EventFeed,
    pub event_q: VecDeque<()>,
    pub balances: (),
    pub orders: (),
    pub positions: (),
    pub strategy: (),
    pub state: State,
}

// pub struct Start;
// pub struct End;

pub struct Commander {
    pub command: Command,
}

impl From<Cerebrum<MarketUpdater>> for Cerebrum<Consumer> {
    fn from(cerebrum: Cerebrum<MarketUpdater>) -> Self {
        todo!()
    }
}

impl Cerebrum<AccountUpdater> {
    fn update(mut self) -> Cerebrum<Consumer> {
        todo!()
    }
}

impl From<Cerebrum<AccountUpdater>> for Cerebrum<Consumer> {
    fn from(cerebrum: Cerebrum<AccountUpdater>) -> Self {
        todo!()
    }
}













#[test]
fn it_works() {
    // Construct dependencies

    // EventFeed
    let (event_tx, event_rx) = mpsc::unbounded_channel();
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