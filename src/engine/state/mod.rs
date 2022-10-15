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

impl<Strategy, Execution> Trader<Strategy, Execution, Initialise> {
    pub fn init(self) -> Engine<Strategy, Execution> {

        // Send ExecutionRequests

        // Wait for response AccountEvents w/ timeout

        // Construct Accounts

        //


        Engine::Consume(Trader::from(self))
    }

}

/// a) Initialise -> Consume
impl<Strategy, Execution> From<Trader<Strategy, Execution, Initialise>> for Trader<Strategy, Execution, Consume> {
    fn from(trader: Trader<Strategy, Execution, Initialise>) -> Self {
        todo!()
    }
}


/// b) Initialise -> Terminate
impl<Strategy, Execution> From<Trader<Strategy, Execution, Initialise>> for Trader<Strategy, Execution, Terminate> {
    fn from(trader: Trader<Strategy, Execution, Initialise>) -> Self {
        todo!()
        // Self {
        //     state: Terminated {
        //
        //     },
        //     feed: trader.feed,
        // }
    }
}
