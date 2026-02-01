## 1.4.3

* Added support for case-insensitive conversion from `&str` for `Locale`:
    * `fn from_str_caseless(&str) -> Option<Self>`
    * `fn from_str_caseless_or_default(&str) -> Self`

## 1.4.2

* Added support for labels of `Locale` variants
* Renamed `iter` -> `iter_variants` in `Locale`
* Added iterators:
    * `fn iter() -> impl Iterator<Item = (Self, &'static str)>`
    * `fn iter_labels() -> impl Iterator<Item = &'static str>`

## 1.4.1

* Added features for serialization and deserialization:
    * `nanoserde_json` — enables `nanoserde::SerJson` and `nanoserde::DeJson` derives for enum `Locale`
    * `nanoserde_binary` — enables `nanoserde::SerBin` and `nanoserde::DeBin` derives for enum `Locale`
    * `nanoserde_ron` — enables `nanoserde::SerRon` and `nanoserde::DeRon` derives for enum `Locale`
* Disabled default features for dependencies

## 1.4.0

* Added support for creating expressions of various types
* Added the constant `DEFAULT` for `Locale`
* Removed the `LocaleTypes` trait

## 1.3.1

* Implemented the `core::fmt::Display` trait for `Locale`
* Added `fn iter() -> impl Iterator<Item = Self>` for `Locale`
* Added features for serialization and deserialization:
    * `serde` — `serde::Serialize` and `serde::Deserialize` derives for enum `Locale`
    * `miniserde` — `miniserde::Serialize` and `miniserde::Deserialize` derives for enum `Locale`
    * `borsh` — `borsh::BorshSerialize` and `borsh::BorshDeserialize` derives for enum `Locale`
* Made functions const:
    * `const fn to_usize(self) -> usize`
    * `const fn from_usize(usize) -> Option<Self>`
    * `const fn to_str(self) -> &'static str`

## 1.3.0

* Created the `LocaleTypes` trait and moved the `Expression` type to its implementation for Locale
* Added function `to_usize` and constant `VARIANTS` for `Locale`
* Added support for conversion between `&str` and `Locale`:
    * `fn to_str(self) -> &'static str`
    * `fn from_str(&str) -> Option<Self>`
    * `fn from_str_or_default(&str) -> Self`
    * `impl From<Locale> for &str`
    * `impl core::str::FromStr for Locale`
    * `impl TryFrom<&str> for Locale`

## 1.2.3

* Changed conversion between `usize` and `Locale`:
    * Removed `unsafe fn from_usize_unchecked(usize) -> Locale`
    * Added:
        * `fn from_usize(usize) -> Option<Locale>`
        * `fn from_usize_or_default(usize) -> Locale`

## 1.2.2

* Added `expressions!` macro to create multiple `Expression` at once

## 1.2.1

* Added support for conversion between `usize` and `Locale`:
    * `unsafe fn from_usize_unchecked(usize) -> Locale`
    * `impl From<Locale> for usize`
    * `impl TryFrom<usize> for Locale`

## 1.2.0

* Split initialization into two macros:
    * With enum `Locale` and type `Expression`: `init_locale!`
    * With current locale storage: `init_locale_with_storage!`

## 1.1.0

* Added `Default` trait implementation for `Locale` (the first locale variant is considered default)
* `Expression` is now initialized as `static` (was previously `const`)
* Added compile-time check to ensure all `Expression` variants are filled
* Isolated the current locale: access is now restricted to `get_locale` and `set_locale`
* Added support for trailing commas in `init_locale!` and `expression!` macros

## 1.0.0

* Initial Release
