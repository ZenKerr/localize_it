/// Initializes a localized expression.
///
/// # Examples
/// 
/// ```rust
/// expression!(HELLO => {
///     EN: "Hello",
///     RU: "Привет",
/// });
/// ```
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
