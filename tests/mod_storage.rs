mod locale;

use crate::locale::{storage, Locale};

#[test]
fn set_and_get() {
    storage::set(Locale::Ru);
    assert_eq!(storage::get(), Locale::Ru);

    storage::set_from_usize(0);
    assert_eq!(storage::get_as_usize(), 0);

    storage::set_from_str("En");
    assert_eq!(storage::get_as_str(), "En");

    storage::set_from_caseless_str("ru");
    assert_eq!(storage::get_as_str(), "Ru");
}
