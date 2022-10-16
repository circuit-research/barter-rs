use crate::engine::error::EngineError;
use barter_data::model::MarketEvent;
use barter_execution::model::AccountEvent;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};


/// Communicates the state of the [`Feed`] as well as the next event.
#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub enum Feed<Event> {
    Next(Event),
    Finished,
}

#[derive(Debug, Clone)]
pub enum Event {
    Market(MarketEvent),
    Account(AccountEvent),
    Command(Command),
}

#[derive(Debug, Clone)]
pub enum Command {
    FetchOpenPositions,
    ExitPosition,
    ExitAllPositions,
    Terminate,
}

pub struct EventFeed {
    pub event_rx: mpsc::UnboundedReceiver<Event>,
}

impl EventFeed {
    pub fn new(event_rx: mpsc::UnboundedReceiver<Event>) -> Self {
        Self { event_rx }
    }

    /// Builder to construct [`EventFeed`] instances.
    pub fn builder() -> EventFeedBuilder {
        EventFeedBuilder::new()
    }

    pub fn next(&mut self) -> Feed<Event> {
        loop {
            match self.event_rx.try_recv() {
                Ok(event) => break Feed::Next(event),
                Err(mpsc::error::TryRecvError::Empty) => continue,
                Err(mpsc::error::TryRecvError::Disconnected) => break Feed::Finished,
            }
        }
    }
}

pub struct EventFeedBuilder {
    market: Option<mpsc::UnboundedReceiver<MarketEvent>>,
    account: Option<mpsc::UnboundedReceiver<AccountEvent>>,
    command: Option<mpsc::UnboundedReceiver<Command>>,
}

impl EventFeedBuilder {
    fn new() -> Self {
        Self {
            market: None,
            account: None,
            command: None,
        }
    }

    pub fn market(self, market: mpsc::UnboundedReceiver<MarketEvent>) -> Self {
        Self {
            market: Some(market),
            ..self
        }
    }

    pub fn account(self, account: mpsc::UnboundedReceiver<AccountEvent>) -> Self {
        Self {
            account: Some(account),
            ..self
        }
    }

    pub fn command(self, command: mpsc::UnboundedReceiver<Command>) -> Self {
        Self {
            command: Some(command),
            ..self
        }
    }

    pub fn build(self) -> Result<EventFeed, EngineError> {
        // Ensure all components are populated
        let mut market_rx = self
            .market
            .ok_or(EngineError::BuilderIncomplete("feed"))?;
        let mut account_rx = self
            .account
            .ok_or(EngineError::BuilderIncomplete("feed"))?;
        let mut command_rx = self
            .command
            .ok_or(EngineError::BuilderIncomplete("feed"))?;

        // Create output EventFeed channel
        let (event_tx, event_rx)  = mpsc::unbounded_channel();

        // Distribute MarketEvents to the EventFeed receiver
        let market_event_tx = event_tx.clone();
        tokio::spawn(async move {
            while let Some(event) = market_rx.recv().await {
                if market_event_tx.send(Event::Market(event)).is_err() {
                    break;
                }
            }
        });

        // Distribute AccountEvents to the EventFeed receiver
        let account_event_tx = event_tx.clone();
        tokio::spawn(async move {
            while let Some(event) = account_rx.recv().await {
                if account_event_tx.send(Event::Account(event)).is_err() {
                    break;
                }
            }
        });

        // Distribute Commands to the EventFeed receiver
        tokio::spawn(async move {
            while let Some(command) = command_rx.recv().await {
                if event_tx.send(Event::Command(command)).is_err() {
                    break;
                }
            }
        });


        Ok(EventFeed::new(event_rx))
    }


}

