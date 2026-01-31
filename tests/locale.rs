mod common;

use common::Locale;

#[test]
fn tests() {
    assert_eq!(Locale::COUNT, 2);
    assert_eq!(Locale::VARIANTS, [Locale::EN, Locale::RU]);
    assert_eq!(Locale::LABELS, ["EN", "RU"]);
    assert_eq!(Locale::DEFAULT, Locale::default());

    assert_eq!(format!("{}", Locale::EN), "EN");

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
