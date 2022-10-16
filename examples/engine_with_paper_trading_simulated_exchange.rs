use barter::{
    engine::Engine,
    event::{EventFeed, Command},
    strategy::OrderGenerator,
    portfolio::MarketUpdater,
    execution::single::ExecutionManager,

};
use barter_integration::model::{Exchange, Instrument, InstrumentKind, Symbol};
use barter_data::{
    ExchangeId,
    builder::Streams,
    model::{MarketEvent, subscription::SubKind},
};
use barter_execution::{
    ExecutionClient, ExecutionId,
    model::{
        balance::Balance,
        order::{Order, RequestCancel, RequestOpen},
    },
    simulated::{
        exchange::{
            SimulatedExchange,
            account::{ClientAccount, balance::ClientBalances},
        },
        execution::SimulatedExecution,
    }
};
use std::{
    time::Duration,
    collections::HashMap
};
use tokio::sync::mpsc;



struct ExampleStrategy;

impl MarketUpdater for ExampleStrategy {
    fn update_from_market(&mut self, market: &MarketEvent) {
        todo!()
    }
}

impl OrderGenerator for ExampleStrategy {
    fn generate_cancels(&self) -> Option<Vec<Order<RequestCancel>>> {
        todo!()
    }

    fn generate_orders(&self) -> Option<Vec<Order<RequestOpen>>> {
        todo!()
    }
}


#[tokio::main]
async fn main() {
    // Define Exchange & Instruments
    let exchange = Exchange::from(ExecutionId::Simulated);
    let instruments = vec![
        Instrument::new("btc", "usdt", InstrumentKind::Spot),
        Instrument::new("eth", "usdt", InstrumentKind::Spot),
    ];

    // Initialise Ftx MarketStream used for SimulatedExchange liquidity
    let market_rx = Streams::builder()
        .subscribe_exchange(
            ExchangeId::Ftx,
            [
                ("btc", "usdt", InstrumentKind::Spot, SubKind::Trade),
                ("eth", "usdt", InstrumentKind::Spot, SubKind::Trade),
            ],
        )
        .init()
        .await
        .expect("failed to initialise MarketStreams")
        .join::<MarketEvent>()
        .await;

    // Initialise SimulatedExchange ExecutionClient
    let (account_tx, account_rx) = mpsc::unbounded_channel();
    let (simulated_tx, simulated_rx) = mpsc::unbounded_channel();

    // Run ExecutionManager containing SimulatedExchange ExecutionClient
    let (execution_tx, execution_rx) = mpsc::unbounded_channel();
    tokio::spawn(
        ExecutionManager {
            exchange: exchange.clone(),
            execution_rx,
            client: SimulatedExecution::init(simulated_tx, account_tx).await,
            event_feed_tx: account_tx.clone()
        }.run()
    );

    // Run SimulatedExchange
    tokio::spawn(
        SimulatedExchange::builder()
            .event_simulated_rx(simulated_rx)
            .account(
                ClientAccount::builder()
                    .latency(Duration::from_millis(50))
                    .fees_percent(0.05)
                    .event_account_tx(event_account_tx)
                    .instruments(instruments.clone())
                    .balances(ClientBalances(
                        HashMap::from([
                            (Symbol::from("btc"), Balance::new(10.0, 10.0)),
                            (Symbol::from("eth"), Balance::new(10.0, 10.0)),
                            (Symbol::from("usdt"), Balance::new(10_000.0, 10_000.0)),
                        ])
                    ))
                    .build()
                    .expect("failed to build ClientAccount"),
            )
            .build()
            .expect("failed to build SimulatedExchange")
            .run()
    );

    // Create channel to distribute user Commands to the Engine (eg/ Command::Terminate)
    let (command_tx, command_rx) = mpsc::unbounded_channel();

    // Build Engine EventFeed for ingesting MarketEvents, AccountEvents, and user Commands
    let feed = EventFeed::builder()
        .market(market_rx)
        .account(account_rx)
        .command(command_rx)
        .build()
        .expect("failed to build EventFeed");

    // Run Engine
    std::thread::spawn(||
        Engine::builder()
            .feed(feed)
            .strategy(ExampleStrategy)
            .execution_tx(execution_tx)
            .instruments(HashMap::from([(exchange, instruments)]))
            .build()
            .expect("failed to build Engine")
            .run()
    );

    // Run system for 60 seconds and then shutdown with Command::Terminate
    tokio::time::sleep(Duration::from_secs(60)).await;
    command_tx.send(Command::Terminate).unwrap();
}