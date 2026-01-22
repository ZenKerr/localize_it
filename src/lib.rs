#![no_std]

#[macro_export]
macro_rules! init_locale {
    ($($variant: ident),+) => {
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

        mod current_locale_storage {
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

        pub use current_locale_storage::{get_locale, set_locale};
    };
}

#[macro_export]
macro_rules! expression {
    ($name: ident => {$($lang: ident: $expression: expr),+}) => {
        pub static $name: [&str; Locale::COUNT] = {
            let mut temp_expression = [None; Locale::COUNT];
            $(
                temp_expression[Locale::$lang as usize] = Some($expression);
            )+

            let mut expression = [""; Locale::COUNT];

            let mut i = 0;
            while i < temp_expression.len() {
                match temp_expression[i] {
                    Some(expression_variant) => expression[i] = expression_variant,
                    None => panic!("Initialize Error: Missing language variant in expression"),
                }

                i += 1;
            }

            expression
        };
    };
}

#[macro_export]
macro_rules! localize {
    ($expression: path) => {
        $expression[get_locale() as usize]
    };
}
