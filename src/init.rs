/// Initializes the localization system.
///
/// This macro must be invoked **once** at the module level. It generates:
/// * `enum Locale` — the list of supported languages;
/// * `type Expression` — a type for localized expressions;
///
/// # Examples
/// 
/// ```rust
/// init_locale!(EN, RU);
/// ```
#[macro_export]
macro_rules! init_locale {
    ($($variant: ident),+ $(,)?) => {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(usize)]
        pub enum Locale {
            #[default]
            $($variant),+
        }

        impl Locale {
            pub const COUNT: usize = [$(stringify!($variant)),+].len();
        }

        pub type Expression = [&'static str; Locale::COUNT];
    };
}

/// Initializes the localization system and the mechanism for storing the current locale.
///
/// This macro must be invoked **once** at the module level. It generates:
/// * `enum Locale` — the list of supported languages;
/// * `type Expression` — a type for localized expressions;
/// * `get_locale()` and `set_locale()` — functions for managing the global current locale state.
///
/// # Examples
/// 
/// ```rust
/// init_locale_with_storage!(EN, RU);
/// ```
#[macro_export]
macro_rules! init_locale_with_storage {
    ($($variant: ident),+ $(,)?) => {
        localize_it::init_locale!($($variant),+);

        mod storage {
            use super::Locale;
            use core::{
                mem::transmute,
                sync::atomic::{AtomicUsize, Ordering},
            };

            static CURRENT_LOCALE: AtomicUsize = AtomicUsize::new(0);

            #[inline]
            pub fn get_locale() -> Locale {
                unsafe { transmute(CURRENT_LOCALE.load(Ordering::Relaxed)) }
            }

            #[inline]
            pub fn set_locale(locale: Locale) {
                CURRENT_LOCALE.store(locale as usize, Ordering::Relaxed)
            }
        }

        pub use storage::{get_locale, set_locale};
    }
}
