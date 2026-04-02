use localize_it::init_locale;
use std::io::{stdin, stdout, Write};

// Define available locales and enable built-in storage
init_locale!(En, Ru, storage = true);

// Define localized expressions (can be any compile-time type)
expressions!(
    ENTER_LANGUAGE => {
        En: "Enter En or Ru: ",
        Ru: "Введите En или Ru: ",
    },
    ENTER_YOUR_NAME => {
        En: "Please, enter your name: ",
        Ru: "Пожалуйста, введите ваше имя: ",
    },
    HELLO: fn(&str) -> String => {
        En: |name: &str| format!("Hello, {name}!"),
        Ru: |name: &str| format!("Привет, {name}!"),
    },
);

// Simple input helper for demonstration purposes
fn input() -> String {
    let mut temp = String::new();

    stdout().flush().unwrap();
    stdin().read_line(&mut temp).unwrap();
    temp.trim().to_string()
}

fn main() {
    // Explicitly use a specific locale
    print!("{}", localize!(ENTER_LANGUAGE, Locale::En));

    let lang = input();

    // Set the locale in storage, falls back to default (En)
    storage::set_from_caseless_str_or_default(&lang);

    // Use the locale from storage
    print!("{}", localize!(ENTER_YOUR_NAME));

    let name = input();

    // Use a callable expression
    println!("{}", localize!(HELLO as (&name)));
}
