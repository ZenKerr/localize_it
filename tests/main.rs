mod locale;
mod locale_with_storage;

#[cfg(test)]
mod tests {
    use localize_it::localize;

    #[test]
    fn locale() {
        use crate::locale::{ui, Locale};

        assert_eq!(localize!(ui::BUTTON_TEXT, Locale::EN), ui::BUTTON_TEXT[0]);
        assert_eq!(localize!(ui::BUTTON_TEXT, Locale::RU), ui::BUTTON_TEXT[1]);
    }

    #[test]
    fn locale_with_storage() {
        use crate::locale_with_storage::{error, get_locale, set_locale, Locale};

        assert_eq!(get_locale(), Locale::EN);
        assert_eq!(localize!(error::ACTION_FAILED), error::ACTION_FAILED[0]);

        set_locale(Locale::RU);

        assert_eq!(get_locale(), Locale::RU);
        assert_eq!(localize!(error::ACTION_FAILED), error::ACTION_FAILED[1]);
    }
}
