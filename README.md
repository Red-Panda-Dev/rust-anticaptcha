# rust-anticaptcha

[![RustLogo.png](https://s.vyjava.xyz/files/2025/01-January/04/1cbd42dd/RustLogo.png)](https://vyjava.xyz/dashboard/image/1cbd42dd-d576-4382-b765-01181b894056)

<hr>

[![Crates.io Version](https://img.shields.io/crates/v/rust-anticaptcha?label=Version&style=flat&color=green)](https://crates.io/crates/rust-anticaptcha)
[![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/rust-anticaptcha?style=flat&label=Downloads&color=blue)](https://crates.io/crates/rust-anticaptcha)
![Crates.io MSRV](https://img.shields.io/crates/msrv/rust-anticaptcha?label=cargo)
[![Static Badge](https://img.shields.io/badge/docs-docs.rs-green?label=Documentation&labelColor=gray)](https://docs.rs/rust-anticaptcha/)


[![Build Doc](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_doc.yml/badge.svg?branch=master)](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_doc.yml)
[![Tests](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/test.yml)
[![Build Dev](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_dev.yml/badge.svg?branch=master)](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_dev.yml)
[![Build Release](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_release.yml/badge.svg?branch=master)](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_release.yml)

Rust library for [AntiCaptcha](https://getcaptchasolution.com/vchfpctqyz) service API.

Tested on UNIX based OS.

The library is intended for software developers and is used to work with the [AntiCaptcha](https://getcaptchasolution.com/agggpuit4b) service API.

## How to install?

We recommend using the latest version of Rust. `rust-anticaptcha` supports Rust 2021.

Install by Cargo command:
```bash
cargo add rust-anticaptcha
```

Add line in your `Cargo.toml` file:
```toml
rust-anticaptcha = "0.1.0"
```

## How to test?

1. Set API_KEY local environment variable:
    ```bash
    export API_KEY=anticaptcha-api-key-variable
    ```
2. Run full tests (with docstrings):
    ```bash
    cargo test
    ```
   Or only project specified tests:
    ```bash
    cargo test --tests
    ```

### Contacts

If you have any questions, please send a message to the [Telegram](https://t.me/pythoncaptcha) chat room.

Or email python-captcha@pm.me
