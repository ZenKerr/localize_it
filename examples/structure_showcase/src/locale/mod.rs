pub mod flag;
pub mod message;

use localize_it::init_locale;

// Initializing locale
init_locale!(En, Ru, path = crate::locale);
