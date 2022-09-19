use barter_data::model::MarketEvent;

#[derive(Debug, Clone)]
pub enum Event {
    Market(MarketEvent),
    Account(AccountEvent),
    Command(Command),
}

#[derive(Debug, Clone)]
pub enum Command {
    Terminate,
    FetchOpenPositions,
    ExitPosition,
    ExitAllPositions,
}