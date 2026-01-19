#![no_std]

#[macro_export]
macro_rules! init_locale {
    ($($variant: ident),+) => {
        use core::sync::atomic::{AtomicUsize, Ordering};
        use core::mem::transmute;

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

        pub static LOCALE: AtomicUsize = AtomicUsize::new(0);

        #[inline]
        pub fn get_locale() -> Locale {
            unsafe {
                transmute(match LOCALE.load(Ordering::Relaxed) {
                    locale if locale < Locale::COUNT => locale,
                    _ => 0,
                })
            }
        }

        #[inline]
        pub fn set_locale(locale: Locale) {
            LOCALE.store(locale as usize, Ordering::Relaxed)
        }
    };
}

#[macro_export]
macro_rules! expression {
    ($name: ident => {$($lang: ident: $expression: expr),+}) => {
        pub static $name: [&str; Locale::COUNT] = {
            let mut expression = [""; Locale::COUNT];

            $(
                expression[Locale::$lang as usize] = $expression;
            )+

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
