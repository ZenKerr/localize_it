mod common;

use crate::common::get_locale_as_usize;
use common::{
    get_locale, get_locale_as_str, set_locale, set_locale_from_caseless_str, set_locale_from_str,
    set_locale_from_usize, Locale,
};

#[test]
fn getters_and_setters() {
    set_locale(Locale::RU);
    assert_eq!(get_locale(), Locale::RU);

    set_locale_from_usize(0);
    assert_eq!(get_locale_as_usize(), 0);

    set_locale_from_str("EN");
    assert_eq!(get_locale_as_str(), "EN");

    set_locale_from_caseless_str("ru");
    assert_eq!(get_locale_as_str(), "RU");
}
