## 1.3.0

* Created trait `LocaleTypes` and move type `Expression` to its implementation for Locale
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
* `Expression` is now initialized as `static` (previously `const`)
* Added compile-time check to ensure all `Expression` variants are filled
* Isolated the current locale: access is now restricted to `get_locale` and `set_locale`
* Added support for trailing commas in `init_locale!` and `expression!` macros

## 1.0.0

* Initial Release
