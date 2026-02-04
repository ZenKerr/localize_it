mod common;

use common::Locale;

#[cfg(feature = "serde")]
#[test]
fn serde() {
    use serde::{Deserialize, Serialize};
    use serde_assert::{Deserializer, Serializer};

    assert_eq!(
        Locale::deserialize(
            &mut Deserializer::builder(
                Locale::En
                    .serialize(&Serializer::builder().build())
                    .unwrap(),
            )
            .build(),
        )
        .unwrap(),
        Locale::En
    );
}

#[cfg(feature = "nanoserde_json")]
#[test]
fn nanoserde_json() {
    use nanoserde::{DeJson, SerJson};

    assert_eq!(
        <Locale as DeJson>::deserialize_json(&SerJson::serialize_json(&Locale::En)).unwrap(),
        Locale::En
    );
}

#[cfg(feature = "nanoserde_binary")]
#[test]
fn nanoserde_binary() {
    use nanoserde::{DeBin, SerBin};

    assert_eq!(
        <Locale as DeBin>::deserialize_bin(&SerBin::serialize_bin(&Locale::En)).unwrap(),
        Locale::En
    );
}

#[cfg(feature = "nanoserde_ron")]
#[test]
fn nanoserde_ron() {
    use nanoserde::{DeRon, SerRon};

    assert_eq!(
        <Locale as DeRon>::deserialize_ron(&SerRon::serialize_ron(&Locale::En)).unwrap(),
        Locale::En
    );
}

#[cfg(feature = "miniserde")]
#[test]
fn miniserde() {
    use miniserde::json::{from_str, to_string};

    assert_eq!(
        from_str::<Locale>(&to_string(&Locale::En)).unwrap(),
        Locale::En
    );
}

#[cfg(feature = "borsh")]
#[test]
fn borsh() {
    use borsh::{from_slice, to_vec};

    assert_eq!(
        from_slice::<Locale>(&to_vec(&Locale::En).unwrap()).unwrap(),
        Locale::En
    );
}
