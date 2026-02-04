mod common;

use crate::common::get_locale_as_usize;
use common::{
    get_locale, get_locale_as_str, set_locale, set_locale_from_caseless_str, set_locale_from_str,
    set_locale_from_usize, Locale,
};

#[test]
fn getters_and_setters() {
    set_locale(Locale::Ru);
    assert_eq!(get_locale(), Locale::Ru);

    set_locale_from_usize(0);
    assert_eq!(get_locale_as_usize(), 0);

    set_locale_from_str("En");
    assert_eq!(get_locale_as_str(), "En");

    set_locale_from_caseless_str("ru");
    assert_eq!(get_locale_as_str(), "Ru");
}
