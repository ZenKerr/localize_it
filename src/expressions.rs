/// Initializes a localized expressions.
///
/// For a single expression, you can use [`expression!`](crate::expression!).
///
/// # Examples
///
/// ```rust
/// expressions!(
///     HELLO => {
///         EN: "Hello",
///         RU: "Привет",
///     },
///     BYE => {
///         EN: "Bye",
///         RU: "Пока",
///     },
/// );
/// ```
/// `EN` and `RU` are codes defined during `Locale` initialization via
/// [`init_locale!`](crate::init_locale!).
#[macro_export]
macro_rules! expressions {
    ($($name: ident => {$($lang: ident: $expression: expr),+ $(,)?}),+ $(,)?) => {
        $(
            localize_it::expression!($name => {$($lang: $expression),+});
        )+
    };
}
