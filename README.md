# localize_it - Simple and fast localization library

![Tests](https://github.com/ZenKerr/localize_it/actions/workflows/rust.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/localize_it.svg)](https://crates.io/crates/localize_it)
[![Documentation](https://docs.rs/localize_it/badge.svg)](https://docs.rs/localize_it)
![License](https://img.shields.io/crates/l/localize_it.svg)

A tiny, zero-allocation localization system for Rust designed with `#![no_std]` support.

This crate provides a macro-based API for defining compile-time locales and localized expressions
without using dynamic memory, hash maps, or runtime lookups. All localized expressions are stored in fixed-size arrays.
You can manually control (via `init_locale!`) the locale state or use the built-in locale storage
(via `init_locale_with_storage!`).

---

## Cargo Features

* `serde` — enables `serde::Serialize` and `serde::Deserialize` derives for `enum Locale`
* `nanoserde_json` — enables `nanoserde::SerJson` and `nanoserde::DeJson` derives for `enum Locale`
* `nanoserde_binary` — enables `nanoserde::SerBin` and `nanoserde::DeBin` derives for `enum Locale`
* `nanoserde_ron` — enables `nanoserde::SerRon` and `nanoserde::DeRon` derives for `enum Locale`
* `miniserde` — enables `miniserde::Serialize` and `miniserde::Deserialize` derives for `enum Locale`
* `borsh` — enables `borsh::BorshSerialize` and `borsh::BorshDeserialize` derives for `enum Locale`

---

## Example

A program that asks the user to choose a language, enter their name, and then greets them in the selected language:

```rust
use localize_it::{expressions, init_locale_with_storage, localize};
use std::io::{stdin, stdout, Write};

// Define available locales
init_locale_with_storage!(EN, RU);

// Expressions can be any type allowed in compile-time context
expressions!(
    ENTER_LANGUAGE => {
        EN: "Enter EN or RU: ",
        RU: "Введите EN или RU: ",
    },
    ENTER_YOU_NAME => {
        EN: "Please, enter your name: ",
        RU: "Пожалуйста, введите ваше имя: ",
    },
    HELLO: fn (&str) -> String => {
        EN: |name: &str| format!("Hello, {name}!"),
        RU: |name: &str| format!("Привет, {name}!"),
    },
);

fn main() {
    let mut lang = String::new();

    // You can set locale manually
    print!("{}", localize!(ENTER_LANGUAGE, Locale::EN));

    stdout().flush().unwrap();
    stdin().read_line(&mut lang).unwrap();

    // Set the selected locale
    set_locale(Locale::from_str_or_default(&lang.trim().to_uppercase()));

    let mut name = String::new();

    // Uses the currently selected locale automatically
    print!("{}", localize!(ENTER_YOU_NAME));

    stdout().flush().unwrap();
    stdin().read_line(&mut name).unwrap();

    // Use callable expression
    println!("{}", localize!(HELLO as (&name.trim())));
}
```

---

## Recommended Project Structure

A recommended way to organize your project is to create a dedicated `locale` module that handles locale
initialization and contains grouped expression modules. For example:

```text
src/
├─ main.rs
└─ locale/
   ├─ mod.rs     # Initialization locale here
   ├─ error.rs   # First module with expressions
   └─ ui.rs      # Second module with expressions
```

---

## Advantages

* O(1) localization lookup
* Zero allocations
* Macro-based API
* No external dependencies without optional features
* `no_std` compatible

---

## Design Constraints

* Not possible to update or add the translations without recompiling
* No plans to add automatic gender agreement, numeral declension, etc

---

## How It Works

* Locales are defined as an `enum` with `#[repr(usize)]`
* Expressions as an array
* Built-in locale storage implemented as `AtomicUsize` with `Relaxed` ordering
* Localization resolves to a simple array index operation

---

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
localize_it = "1.4.1"
```

---

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-MIT))

at your option.
