mod flag;
mod message;

use localize_it::init_locale;

// Initializing locale
init_locale!(En, Ru, path = crate::locale);

// Creating expressions from files
expressions_from_files!(
    {
        En => crate::locale::message::en,
        Ru => crate::locale::message::ru,
    } => [
        HELLO,
        BYE,
    ]
);

// Creating expressions from other files
expressions_from_files!(
    {
        En => crate::locale::flag::en,
        Ru => crate::locale::flag::ru,
    } => [
        IS_ENGLISH: bool,
    ]
);
