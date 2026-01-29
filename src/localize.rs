/// Returns the translation of an expression.
///
/// # Examples
///
/// Using the global locale initialized with
/// [`init_locale_with_storage!`](crate::init_locale_with_storage!):
///
/// ```rust
/// localize!(HELLO)
/// ```
///
/// Specify the locale manually:
///
/// ```rust
/// localize!(HELLO, Locale::EN)
/// ```
/// Locale defined via [`init_locale!`](crate::init_locale!).
#[macro_export]
macro_rules! localize {
    ($expression: path) => {
        $expression[get_locale().to_usize()]
    };

    ($expression: path, $locale: expr) => {
        $expression[$locale.to_usize()]
    };
}
