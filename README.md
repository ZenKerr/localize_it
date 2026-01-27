# localize_it - Simple and fast localization library

![Tests](https://github.com/ZenKerr/localize_it/actions/workflows/rust.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/localize_it.svg)](https://crates.io/crates/localize_it)
[![Documentation](https://docs.rs/localize_it/badge.svg)](https://docs.rs/localize_it)
![License](https://img.shields.io/crates/l/localize_it.svg)

A tiny, zero-allocation localization system for Rust designed with `#![no_std]` support.

This crate provides a macro-based API for defining compile-time locales and localized string expressions
without using dynamic memory, hash maps, or runtime lookups. All localized strings are stored in fixed-size arrays.
You can manually control (via `init_locale!`) the locale state or use the built-in locale storage (via `init_locale_with_storage!`).

---

## Quick Example

With built-in locale storage:

```rust
use localize_it::{init_locale_with_storage, expressions, localize};

init_locale_with_storage!(EN, RU);

expressions!(
    HELLO => {
        EN: "Hello",
        RU: "Привет",
    },
    BYE => {
        EN: "Bye",
        RU: "Пока",
    },
);

fn main() {
    println!("{}", localize!(HELLO));
    println!("{}", localize!(BYE));
}
```

With manual control:

```rust
use localize_it::{init_locale, expression, localize};

init_locale!(EN, RU);

expression!(HELLO => {
    EN: "Hello",
    RU: "Привет",
});

fn main() {
    println!("{}", localize!(HELLO, Locale::EN));
}
```

---

## Features

* O(1) localization lookup
* Zero allocations
* Macro-based API
* No external dependencies
* `no_std` compatible

---

## Design Constraints

* Not possible to update or add the translations without recompiling
* No plans to add automatic gender agreement, numeral declension, etc.

---

## Example With Recommended Project Structure

The recommended usage pattern is to create a dedicated locale module that contains locale initialization
and grouped expression modules.

```text
src/
├─ main.rs
└─ locale/
   ├─ mod.rs     # Locale initialization
   ├─ error.rs   # Expression module for error messages
   └─ ui.rs      # Expression module for UI elements
```

```rust
// mod.rs

pub mod error;
pub mod ui;

use localize_it::init_locale_with_storage;

// Initialize locale with two languages
init_locale_with_storage!(EN, RU);
```

```rust
// error.rs

use super::Expression;
use localize_it::expression;

// Create expression using the macro
expression!(ACTION_FAILED => {
    EN: "Action failed",
    RU: "Действие не выполнено",
});
```

```rust
// ui.rs

use super::Locale;
use localize_it::expression;

// Create another expression using the macro
expression!(BUTTON_TEXT => {
    EN: "Button text",
    RU: "Текст кнопки",
});
```

```rust
// main.rs (or wherever you want to use the localized strings)

use crate::locale::{error, get_locale, set_locale, ui, Locale};
use localize_it::localize;

fn main() {
    let current_locale = get_locale(); // Get current locale
    println!("{:?}", current_locale);

    set_locale(Locale::RU); // Set locale

    // Get one expression
    let error_message = localize!(error::ACTION_FAILED);
    println!("{}", error_message);

    // Get another expression
    let button_text = localize!(ui::BUTTON_TEXT);
    println!("{}", button_text);
}
```

---

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
localize_it = "1.3.0"
```

---

## How It Works

* Locales are defined as an `enum` with `#[repr(usize)]`
* Expressions as an array of `&'static str`
* Built-in locale storage implemented as `AtomicUsize` with `Relaxed` ordering
* Localization resolves to a simple array index operation

---

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-MIT))

at your option.
