/// Returns the translation of an expression for the current locale.
///
/// # Examples
///
/// Specify the locale manually:
///
/// ```rust
/// localize!(HELLO, Locale::EN)
/// ```
///
/// Using the global locale (works only with `init_locale_with_storage`):
///
/// ```rust
/// localize!(HELLO)
/// ```
#[macro_export]
macro_rules! localize {
    ($expression: path, $locale: expr) => {
        $expression[$locale as usize]
    };

    ($expression: path) => {
        $expression[get_locale() as usize]
    };
}
