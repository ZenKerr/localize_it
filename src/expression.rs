/// Initializes a localized expression.
///
/// For multiple expressions, you can use [`expressions!`](crate::expressions!).
///
/// # Examples
///
/// ```rust
/// expression!(HELLO => {
///     EN: "Hello",
///     RU: "Привет",
/// });
/// ```
/// `EN` and `RU` are codes defined during `Locale` initialization via
/// [`init_locale!`](crate::init_locale!).
#[macro_export]
macro_rules! expression {
    ($name: ident => {$($lang: ident: $expression: expr),+ $(,)?}) => {
        pub static $name: [&str; Locale::COUNT] = {
            let mut temp_expression = [None; Locale::COUNT];
            $(
                temp_expression[Locale::$lang as usize] = Some($expression);
            )+

            let mut expression = [""; Locale::COUNT];

            let mut i = 0;
            while i < temp_expression.len() {
                match temp_expression[i] {
                    Some(expression_variant) => expression[i] = expression_variant,
                    None => panic!("Initialize Error: Missing language variant in expression"),
                }

                i += 1;
            }

            expression
        };
    };
}

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
