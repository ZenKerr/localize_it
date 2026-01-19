mod locale;

#[cfg(test)]
mod tests {
    use crate::locale::{error, get_locale, set_locale, ui, Locale};
    use localize_it::localize;

    #[test]
    fn test() {
        assert_eq!(get_locale(), Locale::EN);
        assert_eq!(localize!(error::COOKIE_BUTTON), error::COOKIE_BUTTON[0]);
        assert_eq!(localize!(ui::COOKIE_BUTTON), ui::COOKIE_BUTTON[0]);

        set_locale(Locale::RU);

        assert_eq!(get_locale(), Locale::RU);
        assert_eq!(localize!(error::COOKIE_BUTTON), error::COOKIE_BUTTON[1]);
        assert_eq!(localize!(ui::COOKIE_BUTTON), ui::COOKIE_BUTTON[1]);
    }
}
