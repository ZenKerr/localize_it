mod locale;

use crate::locale::{
    expressions::{TEST, TEST_CALLABLE},
    localize,
};

#[test]
fn localize() {
    let argument = "Test";

    assert_eq!(localize!(TEST), TEST[0]);
    assert_eq!(
        localize!(TEST_CALLABLE as (argument)),
        TEST_CALLABLE[0](argument)
    );
}
