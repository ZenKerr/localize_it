# localize_it

![Tests](https://github.com/ZenKerr/localize_it/actions/workflows/rust.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/localize_it.svg)](https://crates.io/crates/localize_it)
[![Documentation](https://docs.rs/localize_it/badge.svg)](https://docs.rs/localize_it)
![License](https://img.shields.io/crates/l/localize_it.svg)

A tiny, fast localization library with zero runtime dependencies and `#![no_std]` support.

This crate provides a macro-based API for defining compile-time locales and localized
expressions without dynamic memory allocation or hash maps. All localized expressions
are stored as static arrays, enabling localization via simple indexing. The locale can
be managed manually or via the built-in `AtomicUsize` storage with `Relaxed` ordering.

---

## Example

A program that asks the user to choose a language, enter their name, and then greets them in the selected language:

```rust
use localize_it::init_locale;
use std::io::{stdin, stdout, Write};

// Define available locales and enable built-in storage
init_locale!(En, Ru, storage = true);

// Define localized expressions (can be any compile-time type)
expressions!(
    ENTER_LANGUAGE => {
        En: "Enter En or Ru: ",
        Ru: "Введите En или Ru: ",
    },
    ENTER_YOUR_NAME => {
        En: "Please, enter your name: ",
        Ru: "Пожалуйста, введите ваше имя: ",
    },
    HELLO: fn(&str) -> String => {
        En: |name: &str| format!("Hello, {name}!"),
        Ru: |name: &str| format!("Привет, {name}!"),
    },
);

// Simple input helper for demonstration purposes
fn input() -> String {
  let mut temp = String::new();

  stdout().flush().unwrap();
  stdin().read_line(&mut temp).unwrap();
  temp.trim().to_string()
}

fn main() {
  // Use a specific locale manually
  print!("{}", localize!(ENTER_LANGUAGE, Locale::En));

  let lang = input();

  // Set the locale in storage
  storage::set_from_caseless_str(&lang);

  // Use the locale from storage
  print!("{}", localize!(ENTER_YOUR_NAME));

  let name = input();

  // Use a callable expression
  println!("{}", localize!(HELLO as (&name)));
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

## Design Constraints

* Not possible to update or add the translations without recompiling
* No plans to add automatic gender agreement, numeral declension, etc

---

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
localize_it = "1.5.0"
```

---

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ZenKerr/localize_it/blob/HEAD/LICENSE-MIT))

at your option.
