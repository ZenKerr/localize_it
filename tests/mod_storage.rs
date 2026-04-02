mod locale;

use crate::locale::{storage, Locale};

#[test]
fn set_and_get() {
    storage::set(Locale::Ru);
    assert_eq!(storage::get(), Locale::Ru);
}

#[test]
fn set_and_get_usize() {
    assert_eq!(storage::set_from_usize(1), Ok(()));
    assert_eq!(storage::get_as_usize(), 1);

    assert_eq!(storage::set_from_usize(3).is_err(), true);
    assert_eq!(storage::get_as_usize(), 1);

    storage::set_from_usize_or_default(1);
    assert_eq!(storage::get_as_usize(), 1);

    storage::set_from_usize_or_default(3);
    assert_eq!(storage::get_as_usize(), 0);
}

#[test]
fn set_and_get_str() {
    assert_eq!(storage::set_from_str("Ru"), Ok(()));
    assert_eq!(storage::get_as_str(), "Ru");

    assert_eq!(storage::set_from_str("Es").is_err(), true);
    assert_eq!(storage::get_as_str(), "Ru");

    storage::set_from_str_or_default("Ru");
    assert_eq!(storage::get_as_str(), "Ru");

    storage::set_from_str_or_default("Es");
    assert_eq!(storage::get_as_str(), "En");
}

#[test]
fn set_caseless_str() {
    assert_eq!(storage::set_from_caseless_str("Ru"), Ok(()));
    assert_eq!(storage::get_as_str(), "Ru");

    assert_eq!(storage::set_from_caseless_str("Es").is_err(), true);
    assert_eq!(storage::get_as_str(), "Ru");

    storage::set_from_caseless_str_or_default("Ru");
    assert_eq!(storage::get_as_str(), "Ru");

    storage::set_from_caseless_str_or_default("Es");
    assert_eq!(storage::get_as_str(), "En");
}
