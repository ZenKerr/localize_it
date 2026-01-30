mod common;

use common::Locale;

#[test]
fn locale_to_str() {
    assert_eq!(Locale::EN.to_str(), "EN");

    assert_eq!(<&str>::from(Locale::RU), "RU");
}

#[test]
fn locale_from_str() {
    assert_eq!(Locale::from_str("EN"), Some(Locale::EN));
    assert_eq!(Locale::from_str("ES"), None);

    assert_eq!(Locale::from_str_or_default("RU"), Locale::RU);
    assert_eq!(Locale::from_str_or_default("ES"), Locale::EN);

    assert_eq!("EN".parse(), Ok(Locale::EN));
    assert_eq!("ES".parse::<Locale>().is_err(), true);

    assert_eq!(Locale::try_from("EN"), Ok(Locale::EN));
    assert_eq!(Locale::try_from("ES").is_err(), true);
}
