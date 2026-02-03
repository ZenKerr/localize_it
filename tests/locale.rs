mod common;

use common::Locale;

#[test]
fn constants() {
    assert_eq!(Locale::COUNT, 2);
    assert_eq!(Locale::VARIANTS, [Locale::EN, Locale::RU]);
    assert_eq!(Locale::LABELS, ["EN", "RU"]);
    assert_eq!(Locale::DEFAULT, Locale::default());
}

#[test]
fn display() {
    assert_eq!(format!("{}", Locale::EN), "EN");
}

#[test]
fn iterators() {
    for (i, pair) in Locale::iter().enumerate() {
        assert_eq!(pair, (Locale::VARIANTS[i], Locale::LABELS[i]));
    }

    for (i, variant) in Locale::iter_variants().enumerate() {
        assert_eq!(variant, Locale::VARIANTS[i]);
    }

    for (i, label) in Locale::iter_labels().enumerate() {
        assert_eq!(label, Locale::LABELS[i]);
    }
}

#[test]
fn to_usize() {
    assert_eq!(Locale::EN.to_usize(), 0);

    assert_eq!(usize::from(Locale::RU), 1);
}

#[test]
fn from_usize() {
    assert_eq!(Locale::from_usize(0), Some(Locale::EN));
    assert_eq!(Locale::from_usize(7), None);

    assert_eq!(Locale::from_usize_or_default(1), Locale::RU);
    assert_eq!(Locale::from_usize_or_default(7), Locale::EN);

    assert_eq!(Locale::try_from(0), Ok(Locale::EN));
    assert_eq!(Locale::try_from(7).is_err(), true);
}

#[test]
fn to_str() {
    assert_eq!(Locale::EN.to_str(), "EN");

    assert_eq!(<&str>::from(Locale::RU), "RU");
}

#[test]
fn from_str() {
    assert_eq!(Locale::from_str("EN"), Some(Locale::EN));
    assert_eq!(Locale::from_str("ES"), None);

    assert_eq!(Locale::from_str_or_default("RU"), Locale::RU);
    assert_eq!(Locale::from_str_or_default("ES"), Locale::EN);

    assert_eq!("EN".parse(), Ok(Locale::EN));
    assert_eq!("ES".parse::<Locale>().is_err(), true);

    assert_eq!(Locale::try_from("EN"), Ok(Locale::EN));
    assert_eq!(Locale::try_from("ES").is_err(), true);
}

#[test]
fn from_caseless_str() {
    assert_eq!(Locale::from_caseless_str("en"), Some(Locale::EN));
    assert_eq!(Locale::from_caseless_str("es"), None);

    assert_eq!(Locale::from_caseless_str_or_default("ru"), Locale::RU);
    assert_eq!(Locale::from_caseless_str_or_default("es"), Locale::EN);
}
