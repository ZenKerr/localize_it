/// Initializes the localization system.
///
/// This macro must be invoked **once** at the module level. It generates:
/// * `enum Locale` — the list of supported languages, which provides:
///   * The enum derives the following traits: `Debug`, `Default` (the first variant), `Clone`,
///     `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`
///   * `const COUNT: usize` — the number of supported languages
///   * `const VARIANTS: [Self; Self::COUNT]` — an array containing all supported locale variants
///   * `fn iter() -> impl Iterator<Item = Self>` — returns an iterator over all supported locales
///   * Implements the traits: [`LocaleTypes`](crate::LocaleTypes), `core::fmt::Display`
///   * Conversions between `Locale` and `usize`:
///     * `fn to_usize(self) -> usize`
///     * `fn from_usize(usize) -> Option<Self>`
///     * `fn from_usize_or_default(usize) -> Self`
///     * `impl From<Locale> for usize`
///     * `impl TryFrom<usize> for Locale`
///   * Conversions between `Locale` and `&str`:
///     * `fn to_str(self) -> &'static str`
///     * `fn from_str(&str) -> Option<Self>`
///     * `fn from_str_or_default(&str) -> Self`
///     * `impl From<Locale> for &str`
///     * `impl core::str::FromStr for Locale`
///     * `impl TryFrom<&str> for Locale`
///
/// If you want to use the built-in locale storage, you can use
/// [`init_locale_with_storage!`](crate::init_locale_with_storage!).
///
/// # Features
///
/// * `serde` — enables `serde::Serialize` and `serde::Deserialize` derives for `enum Locale`.
///
/// # Examples
///
/// ```rust
/// init_locale!(EN, RU);
/// ```
#[macro_export]
macro_rules! init_locale {
    ($($variant: ident),+ $(,)?) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(usize)]
        pub enum Locale {
            #[default]
            $($variant),+
        }

        impl Locale {
            pub const COUNT: usize = [$(stringify!($variant)),+].len();
            pub const VARIANTS: [Self; Self::COUNT] = [$(Self::$variant),+];

            #[inline]
            pub fn iter() -> impl Iterator<Item = Self> {
                Self::VARIANTS.into_iter()
            }

            #[inline]
            pub fn to_usize(self) -> usize {
                self as usize
            }

            #[inline]
            pub fn from_usize(value: usize) -> Option<Self> {
                match value {
                    $(
                        _ if value == usize::from(Self::$variant) => Some(Self::$variant),
                    )+
                    _ => None,
                }
            }

            #[inline]
            pub fn from_usize_or_default(value: usize) -> Self {
                Self::from_usize(value).unwrap_or_default()
            }

            #[inline]
            pub fn to_str(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => stringify!($variant),
                    )+
                }
            }

            #[inline]
            pub fn from_str(str: &str) -> Option<Self> {
                match str {
                    $(
                        stringify!($variant) => Some(Self::$variant),
                    )+
                    _ => None,
                }
            }

            #[inline]
            pub fn from_str_or_default(str: &str) -> Self {
                Self::from_str(str).unwrap_or_default()
            }
        }

        impl localize_it::LocaleTypes for Locale {
            type Expression = [&'static str; Locale::COUNT];
        }

        impl core::fmt::Display for Locale {
            #[inline]
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str(self.to_str())
            }
        }

        impl From<Locale> for usize {
            #[inline]
            fn from(locale: Locale) -> Self {
                locale.to_usize()
            }
        }

        impl TryFrom<usize> for Locale {
            type Error = &'static str;

            #[inline]
            fn try_from(value: usize) -> Result<Self, Self::Error> {
                Self::from_usize(value).ok_or("Invalid numeric value for Locale")
            }
        }

        impl From<Locale> for &str {
            #[inline]
            fn from(locale: Locale) -> Self {
                locale.to_str()
            }
        }

        impl core::str::FromStr for Locale {
            type Err = &'static str;

            #[inline]
            fn from_str(str: &str) -> Result<Self, Self::Err> {
                Self::from_str(str).ok_or("Invalid locale identifier")
            }
        }

        impl TryFrom<&str> for Locale {
            type Error = &'static str;

            #[inline]
            fn try_from(str: &str) -> Result<Self, Self::Error> {
                Self::from_str(str).ok_or("Invalid locale identifier")
            }
        }
    };
}
