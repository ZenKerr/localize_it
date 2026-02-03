mod common;

use common::{get_locale_as_usize, ENTER_YOUR_NAME, HELLO};
use localize_it::localize;

#[test]
fn tests() {
    let name = "Ivan";

    assert_eq!(localize!(ENTER_YOUR_NAME), ENTER_YOUR_NAME[0]);
    assert_eq!(localize!(HELLO as (name)), HELLO[0](name));
}
