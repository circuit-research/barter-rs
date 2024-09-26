use barter_data::{
    exchange::{bybit::futures::BybitPerpetualsUsd, bybit::spot::BybitSpot},
    streams::Streams,
    subscription::book::OrderBooksL2,
};
use barter_integration::model::instrument::kind::InstrumentKind;
use futures::StreamExt;
use tracing::info;

#[rustfmt::skip]
#[tokio::main]
async fn main() {
    // Initialise INFO Tracing log subscriber
    init_logging();

    // '--> each call to StreamBuilder::subscribe() creates a separate WebSocket connection
    let streams = Streams::<OrderBooksL2>::builder()
        .subscribe([
            (BybitSpot::default(), "btc", "usdt", InstrumentKind::Spot, OrderBooksL2),
        ])
        .subscribe([
            (BybitPerpetualsUsd::default(), "mother", "usdt", InstrumentKind::Perpetual, OrderBooksL2),
        ])
        .init()
        .await
        .unwrap();

    // Select the ExchangeId::BinanceSpot stream
    // Notes:
    //  - Use `streams.select(ExchangeId)` to interact with the individual exchange streams!
    //  - Use `streams.join()` to join all exchange streams into a single mpsc::UnboundedReceiver!
    let mut joined_stream = streams.join_map().await;

    while let Some((exchange, order_book_l2)) = joined_stream.next().await {
        info!("Exchange: {exchange}, MarketEvent<OrderBookL2>: {order_book_l2:?}");
    }
}

// Initialise an INFO `Subscriber` for `Tracing` Json logs and install it as the global default.
fn init_logging() {
    tracing_subscriber::fmt()
        // Filter messages based on the INFO
        .with_env_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // Disable colours on release builds
        .with_ansi(cfg!(debug_assertions))
        // Enable Json formatting
        .json()
        // Install this Tracing subscriber as global default
        .init()
}
