use std::collections::HashMap;
use barter_integration::model::{Instrument, Symbol};
use async_trait::async_trait;
use chrono::Utc;
use hmac::Hmac;
use tokio::sync::mpsc::UnboundedSender;
use super::{
    event::Event,
    exchange::{ClientStatus, ExecutionClient}
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    instruments: Vec<Instrument>,
    api_key: String,
    #[serde(rename = "api_secret", deserialize_with = "de_string_secret_as_hmac")]
    hmac: Hmac<sha2::Sha256>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BinancePair(String);

impl BinancePair {
    pub fn new(base: &Symbol, quote: &Symbol) -> Self {
        Self(format!("{base}{quote}").to_uppercase())
    }
}

// Todo:
//  - Look into rate limit weighting & optimisation
pub struct Binance {
    instruments_map: HashMap<BinancePair, Instrument>,
    api_key: String,
    hmac: Hmac<sha2::Sha256>,
    status: ClientStatus,
    http_client: reqwest::Client,
}

#[async_trait]
impl ExecutionClient for Binance {
    type Config = Config;

    async fn init(config: Self::Config, event_tx: UnboundedSender<Event>) -> Self {
        // Todo:
        //  - Deal with ConnectionStatus
        //  - Validate Config? Or does the ExchangePortal do that for us?

        Self {
            instruments_map: Self::instruments_map(config.instruments),
            api_key: config.api_key,
            hmac: config.hmac,
            status: ClientStatus::Connected,
            http_client: reqwest::Client::new()
        }
    }

    async fn consume(&self, event_tx: UnboundedSender<Event>) -> Result<(), ()> {
        todo!()
    }

    fn connection_status(&self) -> ClientStatus {
        self.status
    }

    async fn fetch_orders_open(&self) -> () {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct FetchOrdersOpenRequest {
            timestamp: i64,
        }

        impl Default for FetchOrdersOpenRequest {
            fn default() -> Self {
                Self { timestamp: Utc::now().timestamp_millis() }
            }
        }



    }

    async fn fetch_balances(&self) -> () {
        todo!()
    }

    async fn open_order(&self) -> () {
        todo!()
    }

    async fn open_order_batch(&self) -> () {
        todo!()
    }

    async fn cancel_order_by_id(&self) -> () {
        todo!()
    }

    async fn cancel_order_by_instrument(&self) -> () {
        todo!()
    }

    async fn cancel_order_by_batch(&self) -> () {
        todo!()
    }

    async fn cancel_orders_all(&self) -> () {
        todo!()
    }
}

impl Binance {
    fn instruments_map(instruments: Vec<Instrument>) -> HashMap<BinancePair, Instrument> {
        instruments
            .into_iter()
            .map(|instrument| {
                (BinancePair::new(&instrument.base, &instrument.quote), instrument)
            })
            .collect()
    }
}

/// Deserialize a String API secret into Hmac.
pub fn de_string_secret_as_hmac<'de, De, Di>(deserializer: De) -> Result<hmac::Hmac<Di>, De::Error>
where
    De: serde::de::Deserializer<'de>,
    Di: hmac::digest::Update
    + hmac::digest::BlockInput
    + hmac::digest::FixedOutput
    + hmac::digest::Reset
    + Default
    + Clone,
    Di::BlockSize: hmac::crypto_mac::generic_array::ArrayLength<u8>,
{
    use hmac::NewMac;

    let data: &[u8] = serde::Deserialize::deserialize(deserializer)
        .map(str::as_bytes)?;

    hmac::Hmac::new_from_slice(data)
        .map_err(|_| serde::de::Error::custom("API secret invalid"))
}