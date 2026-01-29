/// Returns the translation of an expression.
///
/// # Examples
///
/// Using the global locale initialized with
/// [`init_locale_with_storage!`](crate::init_locale_with_storage!):
///
/// ```rust
/// localize!(HELLO);
/// ```
///
/// Specify the locale manually:
///
/// ```rust
/// localize!(HELLO, Locale::EN);
/// ```
///
/// If your expression is callable (e.g., a function), you can invoke it:
/// ```rust
/// let name = "Ivan";
/// localize!(HELLO as (name));
/// ```
///
/// Locale defined via [`init_locale!`](crate::init_locale!).
#[macro_export]
macro_rules! localize {
    ($expression: path $(as ($($arg: expr),* $(,)?))?) => {
        $expression[get_locale().to_usize()]$(($($arg),*))?
    };

    ($expression: path $(as ($($arg: expr),* $(,)?))?, $locale: expr) => {
        $expression[$locale.to_usize()]$(($($arg),*))?
    };
}
