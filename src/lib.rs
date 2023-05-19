//! # bitpanda-csv
//!
//! bitpanda-csv is a Rust library to parse the Bitpanda trades exported as CSV from your trades history.
//!
//! ## Get started
//!
//! ### Add bitpanda-csv to your Cargo.toml 🦀
//!
//! ```toml
//! bitpanda-csv = "^0.2"
//! ```
//!
//! Supported features are:
//!
//! - `no-log`: disable logging
//! - `moveable-feasts` (*default*): enable getters for moveable feasts
//!
//! ### Parse CSV
//!
//! ```rust
//! use bitpanda_csv::{BitpandaTradeParser, Trade};
//! use std::fs::File;
//!
//! fn main() {
//!     let reader = File::open("./test/bitpanda.csv").expect("could not open CSV file");
//!     let trades: Vec<Trade> = BitpandaTradeParser::parse(reader).expect("failed to parse CSV");
//! }
//! ```
//!
//! ### Parse CSV (async)
//!
//! Add to your Cargo.toml the `async` feature.
//! If you don't need the sync stuff, you can disable the default features then.
//!
//! ```rust
//! use bitpanda_csv::{AsyncBitpandaTradeParser, Trade};
//! use tokio::fs::File;
//! use tokio::io::BufReader;
//!
//! #[tokio::main]
//! async fn main() {
//!     let file = File::open("./test/bitpanda.csv").await.expect("could not open CSV file");
//!     let trades = AsyncBitpandaTradeParser::parse(BufReader::new(file))
//!         .await
//!         .unwrap();
//! }
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

#[cfg(feature = "async")]
mod async_parser;
#[cfg(feature = "mock")]
pub(crate) mod mock;

#[cfg(feature = "sync")]
mod parser;
mod trade;

#[cfg(feature = "mock")]
pub use mock::TradeGenerator;

#[cfg(feature = "async")]
pub use async_parser::AsyncBitpandaTradeParser;
#[cfg(feature = "sync")]
pub use parser::BitpandaTradeParser;
pub use trade::{
    Asset, AssetClass, CryptoCurrency, CsvOption, Currency, Fiat, InOut, Metal, Trade,
    TransactionType,
};
