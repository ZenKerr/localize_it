mod common;

use common::Locale;

#[test]
fn locale_to_usize() {
    assert_eq!(Locale::EN.to_usize(), 0);

    assert_eq!(usize::from(Locale::RU), 1);
}

#[test]
fn locale_from_usize() {
    assert_eq!(Locale::from_usize(0), Some(Locale::EN));
    assert_eq!(Locale::from_usize(7), None);

    assert_eq!(Locale::from_usize_or_default(1), Locale::RU);
    assert_eq!(Locale::from_usize_or_default(7), Locale::EN);

    assert_eq!(Locale::try_from(0), Ok(Locale::EN));
    assert_eq!(Locale::try_from(7).is_err(), true);
}
