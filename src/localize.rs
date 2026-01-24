/// Returns the translation of an expression for the current locale.
///
/// # Examples
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
