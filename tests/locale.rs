mod common;

use common::Locale;

#[test]
fn constants() {
    assert_eq!(Locale::COUNT, 2);
    assert_eq!(Locale::VARIANTS, [Locale::En, Locale::Ru]);
    assert_eq!(Locale::LABELS, ["En", "Ru"]);
    assert_eq!(Locale::DEFAULT, Locale::default());
}

#[test]
fn display() {
    assert_eq!(format!("{}", Locale::En), "En");
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
    assert_eq!(Locale::En.to_usize(), 0);

    assert_eq!(usize::from(Locale::Ru), 1);
}

#[test]
fn from_usize() {
    assert_eq!(Locale::from_usize(0), Some(Locale::En));
    assert_eq!(Locale::from_usize(7), None);

    assert_eq!(Locale::from_usize_or_default(1), Locale::Ru);
    assert_eq!(Locale::from_usize_or_default(7), Locale::En);

    assert_eq!(Locale::try_from(0), Ok(Locale::En));
    assert_eq!(Locale::try_from(7).is_err(), true);
}

#[test]
fn to_str() {
    assert_eq!(Locale::En.to_str(), "En");

    assert_eq!(<&str>::from(Locale::Ru), "Ru");
}

#[test]
fn from_str() {
    assert_eq!(Locale::from_str("En"), Some(Locale::En));
    assert_eq!(Locale::from_str("Es"), None);

    assert_eq!(Locale::from_str_or_default("Ru"), Locale::Ru);
    assert_eq!(Locale::from_str_or_default("Es"), Locale::En);

    assert_eq!("En".parse(), Ok(Locale::En));
    assert_eq!("Es".parse::<Locale>().is_err(), true);

    assert_eq!(Locale::try_from("En"), Ok(Locale::En));
    assert_eq!(Locale::try_from("Es").is_err(), true);
}

#[test]
fn from_caseless_str() {
    assert_eq!(Locale::from_caseless_str("en"), Some(Locale::En));
    assert_eq!(Locale::from_caseless_str("es"), None);

    assert_eq!(Locale::from_caseless_str_or_default("ru"), Locale::Ru);
    assert_eq!(Locale::from_caseless_str_or_default("es"), Locale::En);
}
