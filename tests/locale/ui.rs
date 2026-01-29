use super::Locale;
use localize_it::expression;

expression!(OPEN_BUTTON => {
    EN: "Open",
    RU: "Открыть",
});

expression!(CLOSE_BUTTON: fn () -> &'static str => {
    EN: || "Close",
    RU: || "Закрыть",
});
