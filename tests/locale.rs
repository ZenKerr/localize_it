mod common;

use common::Locale;

#[test]
fn tests() {
    assert_eq!(Locale::COUNT, 2);
    assert_eq!(Locale::VARIANTS, [Locale::EN, Locale::RU]);
    assert_eq!(Locale::DEFAULT, Locale::default());

    assert_eq!(format!("{}", Locale::EN), "EN");

    for (i, variant) in Locale::iter().enumerate() {
        assert_eq!(variant, Locale::VARIANTS[i]);
    }
}
