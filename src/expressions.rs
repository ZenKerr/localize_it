/// Initializes localized expressions.
///
/// For a single expression, you can use [`expression!`](crate::expression!).
///
/// # Examples
///
/// ```rust
/// expressions!(
///     HELLO: fn(&str) -> String => {
///         En: |name: &str| format!("Hello, {name}!"),
///         Ru: |name: &str| format!("Привет, {name}!"),
///     },
///     BYE => {
///         En: "Bye",
///         Ru: "Пока",
///     },
/// );
/// ```
///
/// `En` and `Ru` are codes defined during `Locale` initialization via
/// [`init_locale!`](crate::init_locale!).
#[macro_export]
macro_rules! expressions {
    (
        $(
            $name: ident $(: $content_type: ty)? => {
                $(
                    $lang: ident: $content: expr
                ),+ $(,)?
            }
        ),+ $(,)?
    ) => {
        $(
            localize_it::expression!($name $(: $content_type)? => {
                $(
                    $lang: $content
                ),+
            });
        )+
    };
}
