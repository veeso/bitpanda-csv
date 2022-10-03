# bitpanda-csv

<p align="center">~ A parser for the Bitpanda Trades CSV ~</p>
<p align="center">
  <a href="#get-started-">Get started</a>
  ·
  <a href="https://docs.rs/bitpanda-csv" target="_blank">Documentation</a>
</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.1.2 (03/10/2022)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/bitpanda-csv/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/bitpanda-csv.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/bitpanda-csv"
    ><img
      src="https://img.shields.io/crates/d/bitpanda-csv.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/bitpanda-csv"
    ><img
      src="https://img.shields.io/crates/v/bitpanda-csv.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/bitpanda-csv/actions"
    ><img
      src="https://github.com/veeso/bitpanda-csv/workflows/Build/badge.svg"
      alt="Build CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/bitpanda-csv"
    ><img
      src="https://coveralls.io/repos/github/veeso/bitpanda-csv/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/bitpanda-csv"
    ><img
      src="https://docs.rs/bitpanda-csv/badge.svg"
      alt="Docs"
  /></a>
</p>

---

- [bitpanda-csv](#bitpanda-csv)
  - [About bitpanda-csv 🐼](#about-bitpanda-csv-)
  - [Get started 🏁](#get-started-)
    - [Add bitpanda-csv to your Cargo.toml 🦀](#add-bitpanda-csv-to-your-cargotoml-)
    - [Parse CSV](#parse-csv)
  - [Documentation 📚](#documentation-)
  - [Support the developer ☕](#support-the-developer-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About bitpanda-csv 🐼

bitpanda-csv is a Rust library to parse the Bitpanda trades exported as CSV from your trades history.

---

## Get started 🏁

### Add bitpanda-csv to your Cargo.toml 🦀

```toml
bitpanda-csv = "^0.1.0"
```

### Parse CSV

```rust
use bitpanda_csv::{BitpandaTradeParser, Trade};
use std::fs::File;

fn main() {
    let reader = File::open("./test/bitpanda.csv").expect("could not open CSV file");
    let trades: Vec<Trade> = BitpandaTradeParser::parse(reader).expect("failed to parse CSV");
}
```

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/bitpanda-csv>

---

## Support the developer ☕

If you like bitpanda-csv and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)
[![bitcoin](https://img.shields.io/badge/Bitcoin-ff9416?style=for-the-badge&logo=bitcoin&logoColor=white)](https://btc.com/bc1qvlmykjn7htz0vuprmjrlkwtv9m9pan6kylsr8w)
[![litecoin](https://img.shields.io/badge/Litecoin-345d9d?style=for-the-badge&logo=Litecoin&logoColor=white)](https://blockchair.com/litecoin/address/ltc1q89a7f859gt7nuekvnuuc25wapkq2f8ny78mp8l)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve bitpanda-csv, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View bitpanda-csv's changelog [HERE](CHANGELOG.md)

---

## License 📃

bitpanda-csv is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
