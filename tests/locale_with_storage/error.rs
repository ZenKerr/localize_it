use super::Locale;
use localize_it::expressions;

expressions!(
    FILE_NOT_SELECTED => {
        EN: "File not selected",
        RU: "Файл не выбран",
    },
    OPEN_FILE: fn (&str) -> String => {
        EN: |filename: &str| format!("Can't open file: {filename}"),
        RU: |filename: &str| format!("Не получилось открыть файл: {filename}"),
    },
);
