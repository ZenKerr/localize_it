/// Initializes a localized expression.
///
/// For multiple expressions, you can use [`expressions!`](crate::expressions!).
///
/// # Examples
///
/// You can define simple expressions using:
///
/// ```rust
/// expression!(HELLO => {
///     EN: "Hello",
///     RU: "Привет",
/// });
/// ```
///
/// You can also specify an explicit type (it must be allowed in a compile-time context):
///
/// ```rust
/// expression!(HELLO: fn (&str) -> String => {
///     EN: |name: &str| format!("Hello, {name}!"),
///     RU: |name: &str| format!("Привет, {name}!"),
/// });
/// ```
///
/// `EN` and `RU` are codes defined during `Locale` initialization via
/// [`init_locale!`](crate::init_locale!).
#[macro_export]
macro_rules! expression {
    (
        $name: ident => {
            $(
                $lang: ident: $expression: expr
            ),+ $(,)?
        }
    ) => {
        localize_it::expression!($name: &'static str => {$($lang: $expression),+});
    };

    (
        $name: ident: $content_type: ty => {
            $(
                $lang: ident: $content: expr
            ),+ $(,)?
        }
    ) => {
        pub static $name: [$content_type; Locale::COUNT] = {
            let mut expression = [$($content),+];
            let mut empty = [true; Locale::COUNT];

            $(
                let i = Locale::$lang.to_usize();

                if empty[i] {
                    expression[i] = $content;
                    empty[i] = false;
                } else {
                    panic!(concat!(
                        "Initialize Error: Locale variant ",
                        stringify!($lang),
                        " is duplicated"
                    ));
                }
            )+

            expression
        };
    };
}
