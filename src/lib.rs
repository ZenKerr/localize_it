#![doc = include_str!("../README.md")]

mod arguments;
mod backend;
mod input;
mod names;
mod parts;

use backend::backend;
use proc_macro::TokenStream;

/// Initializes the localization system.
///
/// # Arguments
///
/// * **Unnamed arguments** — locale variants. At least one must be provided.
///   Optionally, a custom label can be specified using `=>`. If no label is
///   provided, it defaults to the variant name.
/// * **storage** — whether to generate storage for the current locale. Default to `false`.
/// * **path** — path to the module where the macro is invoked. Used for resolving
///   paths in the generated code. It is recommended to always specify this,
///   otherwise required imports for generated items may need to be added manually.
/// * **derive** — a list of additional derives applied to the generated `enum Locale`.
///   Defaults to empty.
///
/// # Examples with all features
///
/// ```rust
/// init_locale!(
///     En => "English",
///     Ru => "Russian",
///     storage = true,
///     path = crate::locale,
///     derive = [Deserialize, Serialize],
/// );
/// ```
///
/// # Generated code
///
/// `enum Locale` — core of all system.
///
/// ```rust
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, /* derives from `derive` */)]
/// #[repr(usize)]
/// pub enum Locale { /* provided locale variants */ }
///
/// impl Locale {
///     pub const COUNT: usize = /* number of locale variants */;
///     pub const VARIANTS: [Self; Self::COUNT] = /* array of locale variants */;
///     pub const LABELS: [&'static str; Self::COUNT] = /* array of locale variants labels */;
///     pub const DEFAULT: Self = /* first locale variant */;
///
///     // Iterates over pairs of (variant, label)
///     pub fn iter() -> impl Iterator<Item=(Self, &'static str)> { /* ... */ }
///
///     // Iterates over all variants
///     pub fn iter_variants() -> impl Iterator<Item=Self> { /* ... */ }
///
///     // Iterates over all labels
///     pub fn iter_labels() -> impl Iterator<Item=&'static str> { /* ... */ }
///
///     // Converts the locale to `usize`
///     pub const fn to_usize(self) -> usize { /* ... */ }
///
///     // Converts from `usize`. Returns `None` if the value is invalid.
///     pub const fn from_usize(value: usize) -> Option<Self> { /* ... */ }
///
///     // Converts from `usize`. Uses the default value if invalid.
///     pub fn from_usize_or_default(value: usize) -> Self { /* ... */ }
///
///     // Converts the locale to `&str`
///     pub const fn to_str(self) -> &'static str { /* ... */ }
///
///     // Converts from `&str`. Returns `None` if the value is invalid.
///     pub fn from_str(str: &str) -> Option<Self> { /* ... */ }
///
///     // Converts from `&str`. Uses the default value if invalid.
///     pub fn from_str_or_default(str: &str) -> Self { /* ... */ }
///
///     // Converts from `&str`, ignoring ASCII case. Returns `None` if invalid.
///     pub const fn from_caseless_str(str: &str) -> Option<Self> { /* ... */ }
///
///     // Converts from `&str`, ignoring ASCII case. Uses the default value if invalid.
///     pub fn from_caseless_str_or_default(str: &str) -> Self { /* ... */ }
/// }
///
/// impl Default for Locale { /* ... */ }
/// impl core::fmt::Display for Locale { /* ... */ }
/// impl From<Locale> for usize { /* ... */ }
/// impl TryFrom<usize> for Locale { /* ... */ }
/// impl From<Locale> for &str { /* ... */ }
/// impl core::str::FromStr for Locale { /* ... */ }
/// impl TryFrom<&str> for Locale { /* ... */ }
/// ```
///
/// `mod storage` — generated only if `storage = true`.
///
/// ```rust
/// pub mod storage {
///     use super::Locale;
///     use core::sync::atomic::{AtomicUsize, Ordering};
///
///     // The current locale. Access is only available through the functions below
///     static CURRENT_LOCALE: AtomicUsize = AtomicUsize::new(Locale::DEFAULT.to_usize());
///
///     // Returns the current locale
///     pub fn get() -> Locale { /* ... */ }
///
///     // Sets the current locale
///     pub fn set(locale: Locale) { /* ... */ }
///
///     // Returns the current locale as `usize`
///     pub fn get_as_usize() -> usize { /* ... */ }
///
///     // Sets the current locale from `usize`
///     // If the value is invalid, the locale falls back to the default.
///     pub fn set_from_usize(value: usize) { /* ... */ }
///
///     // Returns the current locale as `&str`.
///     pub fn get_as_str() -> &'static str { /* ... */ }
///
///     // Sets the current locale from `&str`.
///     // If the value is invalid, the locale falls back to the default.
///     pub fn set_from_str(str: &str) { /* ... */ }
///
///     // Sets the current locale from `&str`, ignoring ASCII case.
///     // If the value is invalid, the locale falls back to the default.
///     pub fn set_from_caseless_str(str: &str) { /* ... */ }
/// }
/// ```
///
/// `expression!` — a macro for defining localized expressions.
///
/// ```rust
/// expression!(HELLO => {
///     En: "Hello",
///     Ru: "Привет",
/// });
/// ```
///
/// The expression can be any compile-time type, including functions.
/// For any type other than `&str`, the type must be specified explicitly.
///
/// ```rust
/// expression!(HELLO: fn(&str) -> String => {
///     En: |name: &str| format!("Hello, {name}!"),
///     Ru: |name: &str| format!("Привет, {name}!"),
/// });
/// ```
///
/// `expressions!` — similar to `expression!`, but allows defining multiple expressions at once.
///
/// ```rust
/// expressions!(
///     HELLO => {
///         En: "Hello",
///         Ru: "Привет",
///     },
///     HELLO_WITH_NAME: fn(&str) -> String => {
///         En: |name: &str| format!("Hello, {name}!"),
///         Ru: |name: &str| format!("Привет, {name}!"),
///     },
/// );
/// ```
///
/// `localize!` — a macro for retrieving a localized value.
///
/// If `storage = true` is enabled, the macro can be used without explicitly
/// specifying a locale — the current locale will be used automatically.
///
/// ```rust
/// localize!(HELLO);
/// ```
///
/// A locale can also be provided manually.
///
/// ```rust
/// localize!(HELLO, Locale::En);
/// ```
///
/// If the expression is callable, it can be invoked with arguments.
///
/// ```rust
/// let name = "Ivan";
/// localize!(HELLO_WITH_NAME as (name));
/// ```
#[proc_macro]
pub fn init_locale(input: TokenStream) -> TokenStream {
    backend(input).unwrap_or_else(|error| error.to_compile_error().into())
}
