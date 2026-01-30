mod common;

use common::{get_locale, set_locale, Locale, ENTER_YOU_NAME, HELLO};
use localize_it::localize;

#[test]
fn localize_and_storage() {
    let name = "Иван";

    assert_eq!(get_locale(), Locale::EN);
    assert_eq!(localize!(ENTER_YOU_NAME), ENTER_YOU_NAME[0]);

    set_locale(Locale::RU);

    assert_eq!(get_locale(), Locale::RU);
    assert_eq!(localize!(HELLO as (name)), HELLO[1](name));
}
