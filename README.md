# localize_it – Simple and fast localization library

![Tests](https://github.com/ZenKerr/localize_it/actions/workflows/rust.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/localize_it.svg)](https://crates.io/crates/localize_it)
[![Documentation](https://docs.rs/localize_it/badge.svg)](https://docs.rs/localize_it)
![License](https://img.shields.io/crates/l/localize_it.svg)

A tiny, zero-allocation localization system for Rust designed with `#![no_std]` support.

This crate provides a macro-based API for defining compile-time locales and localized string expressions
without using dynamic memory, hash maps, or runtime lookups. All localized strings are stored in fixed-size arrays,
and the active locale is stored in an atomic variable, making access extremely fast and thread-safe.

---

## Quick Example

```rust
use localize_it::{init_locale, expression, localize};

init_locale!(EN, RU);

expression!(HELLO => {
    EN: "Hello",
    RU: "Привет",
});

fn main() {
    println!("{}", localize!(HELLO));
}
```

---

## Features

* O(1) localization lookup
* Zero allocations
* `no_std` compatible
* Thread-safe global locale (`AtomicUsize`)
* Macro-based API
* No external dependencies

---

## Design Constraints

- Locales are defined at compile time
- Expressions must be `&'static str`

---

## Example With Recommended Project Structure

The recommended usage pattern is to create a dedicated locale module that contains locale initialization and grouped expression modules.
For example:

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

use localize_it::init_locale;

// Initialize locale with two languages
init_locale!(EN, RU);
```

```rust
// error.rs

use crate::locale::Expression;

// Expressions can be created manually without the macro
pub const COOKIE_BUTTON: Expression = [
    "Sorry, the button doesn't work",
    "Извините, кнопка не работает",
];
```

```rust
// ui.rs

use crate::locale::Locale;
use localize_it::expression;

// Recommended way to create expressions using the macro
expression!(COOKIE_BUTTON => {
    EN: "Click to get a cookie",
    RU: "Нажмите, чтобы получить печенье",
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
    let button_error = localize!(error::COOKIE_BUTTON);
    println!("{}", button_error);

    // Get another expression
    let button_label = localize!(ui::COOKIE_BUTTON);
    println!("{}", button_label);
}
```

---

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
localize_it = "1.0.0"
```

---

## How It Works

- Locales are defined as an `enum` with `#[repr(usize)]`
- Each localized string is compiled into a fixed-size `&'static str` array
- The active locale is stored as an atomic integer
- Localization resolves to a simple array index operation
- Invalid locale values safely fall back to the first locale variant, preventing undefined behavior.

---

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-MIT))

at your option.
