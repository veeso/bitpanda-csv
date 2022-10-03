//! # bitpanda-csv
//!
//! bitpanda-csv is a Rust library to parse the Bitpanda trades exported as CSV from your trades history.
//!
//! ## Get started
//!
//! ### Add bitpanda-csv to your Cargo.toml 🦀
//!
//! ```toml
//! bitpanda-csv = "^0.1.0"
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

#![doc(html_playground_url = "https://play.rust-lang.org")]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod parser;
mod trade;

pub use parser::BitpandaTradeParser;
pub use trade::{
    Asset, AssetClass, CryptoCurrency, CsvOption, Currency, Fiat, InOut, Metal, Trade,
    TransactionType,
};
