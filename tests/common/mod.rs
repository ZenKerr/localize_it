use localize_it::{expressions, init_locale_with_storage};

init_locale_with_storage!(En, Ru);

expressions!(
    ENTER_YOUR_NAME => {
        En: "Please, enter your name: ",
        Ru: "Пожалуйста, введите ваше имя: ",
    },
    HELLO: fn(&str) -> String => {
        En: |name: &str| format!("Hello, {name}!"),
        Ru: |name: &str| format!("Привет, {name}!"),
    },
);
