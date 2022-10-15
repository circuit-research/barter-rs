use crate::event::EventFeed;

pub mod state;



pub enum Engine {
    Consumer,
    MarketUpdater,
    OrderGeneratorAlgorithmic,
    OrderGeneratorManual,
    AccountUpdater,
    Commander,
    Terminated
}

pub struct Trader<State> {
    pub state: State,
    pub feed: EventFeed,
}

struct Consumer;
struct MarketUpdater;