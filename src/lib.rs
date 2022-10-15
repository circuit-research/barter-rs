#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    rust_2018_idioms,
    // missing_docs
)]

//! # Barter
//! [`Barter`] is an open-source Rust framework for building **event-driven live-trading & backtesting systems**.
//! Algorithmic trade with the peace of mind that comes from knowing your strategies have been
//! backtested with a near-identical trading Engine.
//! It is:
//! * **Fast**: Barter provides a multi-threaded trading Engine framework built in high-performance Rust (in-rust-we-trust).
//! * **Easy**: Barter provides a modularised data architecture that focuses on simplicity.
//! * **Customisable**: A set of traits define how every Barter component communicates, providing a highly extensible
//! framework for trading.
//!
//! See [`Readme`].

/// Todo:
pub mod event;
pub mod data;
pub mod execution;
pub mod statistic;
pub mod engine;
