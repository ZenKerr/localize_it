mod locale;
mod locale_with_storage;

#[cfg(test)]
mod tests {
    use localize_it::localize;

    #[cfg(feature = "serde")]
    #[test]
    fn locale_serde() {
        use crate::locale::Locale;
        use serde::{Deserialize, Serialize};
        use serde_assert::{Deserializer, Serializer};

        assert_eq!(
            Locale::deserialize(
                &mut Deserializer::builder(
                    Locale::EN
                        .serialize(&Serializer::builder().build())
                        .unwrap(),
                )
                .build(),
            )
            .unwrap(),
            Locale::EN
        );
    }

    #[cfg(feature = "miniserde")]
    #[test]
    fn locale_miniserde() {
        use crate::locale::Locale;
        use miniserde::json::{from_str, to_string};

        assert_eq!(
            from_str::<Locale>(&to_string(&Locale::EN)).unwrap(),
            Locale::EN
        );
    }

    #[cfg(feature = "borsh")]
    #[test]
    fn locale_borsh() {
        use crate::locale::Locale;
        use borsh::{from_slice, to_vec};

        assert_eq!(
            from_slice::<Locale>(&to_vec(&Locale::EN).unwrap()).unwrap(),
            Locale::EN
        );
    }

    #[test]
    fn locale_const() {
        use crate::locale::Locale;

        assert_eq!(Locale::COUNT, 2);

        assert_eq!(Locale::VARIANTS, [Locale::EN, Locale::RU]);

        assert_eq!(Locale::DEFAULT, Locale::default());
    }

    #[test]
    fn locale_iter() {
        use crate::locale::Locale;

        for (i, variant) in Locale::iter().enumerate() {
            assert_eq!(variant, Locale::VARIANTS[i]);
        }
    }

    #[test]
    fn locale_display() {
        use crate::locale::Locale;

        assert_eq!(format!("{}", Locale::EN), "EN");
    }

    #[test]
    fn locale_to_usize() {
        use crate::locale::Locale;

        assert_eq!(Locale::EN.to_usize(), 0);

        assert_eq!(usize::from(Locale::RU), 1);
    }

    #[test]
    fn locale_from_usize() {
        use crate::locale::Locale;

        assert_eq!(Locale::from_usize(0), Some(Locale::EN));
        assert_eq!(Locale::from_usize(7), None);

        assert_eq!(Locale::from_usize_or_default(1), Locale::RU);
        assert_eq!(Locale::from_usize_or_default(7), Locale::EN);

        assert_eq!(Locale::try_from(0), Ok(Locale::EN));
        assert_eq!(Locale::try_from(7).is_err(), true);
    }

    #[test]
    fn locale_to_str() {
        use crate::locale::Locale;

        assert_eq!(Locale::EN.to_str(), "EN");

        assert_eq!(<&str>::from(Locale::RU), "RU");
    }

    #[test]
    fn locale_from_str() {
        use crate::locale::Locale;

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
    fn locale() {
        use crate::locale::{ui, Locale};

        assert_eq!(localize!(ui::OPEN_BUTTON, Locale::EN), ui::OPEN_BUTTON[0]);
        assert_eq!(
            localize!(ui::CLOSE_BUTTON as (), Locale::RU),
            ui::CLOSE_BUTTON[1]()
        );
    }

    #[test]
    fn locale_with_storage() {
        use crate::locale_with_storage::{error, get_locale, set_locale, Locale};

        let filename = "test.txt";

        assert_eq!(get_locale(), Locale::EN);
        assert_eq!(
            localize!(error::FILE_NOT_SELECTED),
            error::FILE_NOT_SELECTED[0]
        );

        set_locale(Locale::RU);

        assert_eq!(get_locale(), Locale::RU);
        assert_eq!(
            localize!(error::OPEN_FILE as (filename)),
            error::OPEN_FILE[1](filename)
        );
    }
}
