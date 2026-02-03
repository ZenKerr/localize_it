use localize_it::{expressions, init_locale_with_storage};

init_locale_with_storage!(EN, RU);

expressions!(
    ENTER_YOUR_NAME => {
        EN: "Please, enter your name: ",
        RU: "Пожалуйста, введите ваше имя: ",
    },
    HELLO: fn (&str) -> String => {
        EN: |name: &str| format!("Hello, {name}!"),
        RU: |name: &str| format!("Привет, {name}!"),
    },
);
