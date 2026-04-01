pub mod expressions;

use localize_it::init_locale;

init_locale!(En, Ru, storage = true, path = crate::locale);
