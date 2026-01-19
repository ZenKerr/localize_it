# localize_it – Simple and fast library for localization

A tiny, zero-allocation localization system for Rust designed with `#![no_std]` support.

This crate provides a macro-based API for defining compile-time locales and localized string expressions
without using dynamic memory, hash maps, or runtime lookups. All localized strings are stored in fixed-size arrays,
and the active locale is stored in an atomic variable, making access extremely fast and thread-safe.

---

## Features

* **O(1) localization lookup**
* **Zero allocations**
* **`no_std` compatible**
* **Thread-safe global locale** (`AtomicUsize`)
* **Macro-based API**
* **No external dependencies**

---

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
localize_it = "1.0.0"
```

---

## Example

The recommended usage pattern is to create a dedicated locale module that contains locale initialization and grouped expression modules.
For example:

```text
src/
├─ main.rs
└─ locale/
   ├─ mod.rs     # Locale initialization
   ├─ error.rs   # Expression package for error messages
   └─ ui.rs      # Expression package for UI elements
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
expression!(COOKIE_BUTTON => {EN: "Click to get a cookie", RU: "Нажмите, чтобы получить печенье"});
```

```rust
// main.rs (or wherever you want to use the localized strings)

use crate::locale::error;
use crate::locale::ui;
use crate::locale::{get_locale, set_locale};
use localize_it::localize;

fn main() {
    let current_locale = get_locale(); // Get current locale

    set_locale(Locale::RU); // Set locale

    localize!(error::COOKIE_BUTTON); // Get one expression
    localize!(ui::COOKIE_BUTTON); // Get another expression
}
```

---

## How It Works

- Locales are defined as an `enum` with `#[repr(usize)]`
- Each localized string is compiled into a fixed-size `&'static str` array
- The active locale is stored as an atomic integer
- Localization resolves to a simple array index operation
- Invalid locale values safely fall back to the first locale variant

---

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-MIT))

at your option.
