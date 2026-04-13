mod en;
mod ru;

use localize_it::init_locale;

// Initializing locale
init_locale!(En, Ru, path = crate::locale);

// Creating expressions from files
expressions_from_files!(
    {
        En => crate::locale::en,
        Ru => crate::locale::ru,
    } => [
        HELLO,
        IS_ENGLISH: bool,
    ]
);
