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

impl Trader<Initialise> {
    pub fn init(self) -> Engine {










        Engine::Consume(Trader::from(self))
    }

}

/// a) Initialise -> Consume
impl From<Trader<Initialise>> for Trader<Consume> {
    fn from(trader: Trader<Initialise>) -> Self {
        todo!()
    }
}


/// b) Initialise -> Terminate
impl From<Trader<Initialise>> for Trader<Terminate> {
    fn from(trader: Trader<Initialise>) -> Self {
        todo!()
        // Self {
        //     state: Terminated {
        //
        //     },
        //     feed: trader.feed,
        // }
    }
}
