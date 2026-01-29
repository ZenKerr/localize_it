#[doc(hidden)]
#[macro_export]
macro_rules! __init_locale_default_const {
    ($default_variant:ident $(, $_: ident)*) => {
        pub const DEFAULT: Self = Self::$default_variant;
    };
}
