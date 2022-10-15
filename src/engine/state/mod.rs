use self::{
    consume::Consume,
    terminate::Terminate,
};
use crate::engine::{Engine, Trader};

pub mod consume;
pub mod market;
pub mod order;
pub mod account;
pub mod command;
pub mod terminate;

/// [`Initialise`] can transition to one of:
/// a) [`Consumer`]
/// b) [`Terminate`]
pub struct Initialise;

impl<Strategy> Trader<Strategy, Initialise> {
    pub fn init(self) -> Engine<Strategy> {
        // Send ExecutionRequests

        // Wait for response AccountEvents w/ timeout

        // Construct Accounts

        //


        Engine::Consume(Trader::from(self))
    }
}

/// a) Initialise -> Consume
impl<Strategy> From<Trader<Strategy, Initialise>> for Trader<Strategy, Consume> {
    fn from(trader: Trader<Strategy, Initialise>) -> Self {
        todo!()
    }
}


/// b) Initialise -> Terminate
impl<Strategy> From<Trader<Strategy, Initialise>> for Trader<Strategy, Terminate> {
    fn from(trader: Trader<Strategy, Initialise>) -> Self {
        todo!()
        // Self {
        //     state: Terminated {
        //
        //     },
        //     feed: trader.feed,
        // }
    }
}
