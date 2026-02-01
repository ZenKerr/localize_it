mod default_const;

/// Initializes the localization system.
///
/// This macro must be invoked **once** at the module level.
///
/// It generates an `enum Locale` defining the supported languages and providing:
/// * The enum derives the following traits: `Debug`, `Default` (the first variant), `Clone`,
///   `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`
/// * Constants:
///   * `COUNT: usize` — the number of supported languages
///   * `VARIANTS: [Self; Self::COUNT]` — an array of supported locale variant identifiers
///   * `LABELS: [&'static str; Self::COUNT]` — an array of locale variants labels
///   * `DEFAULT: Self` — a const-compatible equivalent of `Default::default()`
/// * Iterators:
///   * `fn iter() -> impl Iterator<Item = (Self, &'static str)>` — returns an iterator over variant-label pairs
///   * `fn iter_variants() -> impl Iterator<Item = Self>` — returns an iterator over VARIANTS
///   * `fn iter_labels() -> impl Iterator<Item = &'static str>` — returns an iterator over LABELS
/// * Implements `core::fmt::Display`
/// * Conversions between `Locale` and `usize`:
///   * `const fn to_usize(self) -> usize`
///   * `const fn from_usize(usize) -> Option<Self>`
///   * `fn from_usize_or_default(usize) -> Self`
///   * `impl From<Locale> for usize`
///   * `impl TryFrom<usize> for Locale`
/// * Conversions between `Locale` and `&str` (using locale identifiers, not labels):
///   * `const fn to_str(self) -> &'static str`
///   * `fn from_str(&str) -> Option<Self>`
///   * `fn from_str_or_default(&str) -> Self`
///   * `impl From<Locale> for &str`
///   * `impl core::str::FromStr for Locale`
///   * `impl TryFrom<&str> for Locale`
///   * And with ASCII case-insensitive matching (locale identifiers are usually ASCII):
///     * `fn from_str_caseless(&str) -> Option<Self>`
///     * `fn from_str_caseless_or_default(&str) -> Self`
///
/// If you want to use the built-in locale storage, you can use
/// [`init_locale_with_storage!`](crate::init_locale_with_storage!).
///
/// # Features
///
/// * `serde` — enables `serde::Serialize` and `serde::Deserialize` derives for `enum Locale`
/// * `nanoserde_json` — enables `nanoserde::SerJson` and `nanoserde::DeJson` derives for
///   `enum Locale`
/// * `nanoserde_binary` — enables `nanoserde::SerBin` and `nanoserde::DeBin` derives for
///   `enum Locale`
/// * `nanoserde_ron` — enables `nanoserde::SerRon` and `nanoserde::DeRon` derives for
///   `enum Locale`
/// * `miniserde` — enables `miniserde::Serialize` and `miniserde::Deserialize` derives for
///   `enum Locale`
/// * `borsh` — enables `borsh::BorshSerialize` and `borsh::BorshDeserialize` derives for
///   `enum Locale`
///
/// # Examples
///
/// Default initialization (labels are identical to locale identifiers):
///
/// ```rust
/// init_locale!(EN, RU);
/// ```
///
/// If you need human-readable labels (e.g., for a language selector UI):
///
/// ```rust
/// init_locale!(EN => "English", RU => "Русский");
/// ```
#[macro_export]
macro_rules! init_locale {
    ($($variant: ident),+ $(,)?) => {
        localize_it::init_locale!($($variant => stringify!($variant)),+);
    };

    ($($variant: ident => $label: expr),+ $(,)?) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "nanoserde_json", derive(nanoserde::SerJson, nanoserde::DeJson))]
        #[cfg_attr(feature = "nanoserde_binary", derive(nanoserde::SerBin, nanoserde::DeBin))]
        #[cfg_attr(feature = "nanoserde_ron", derive(nanoserde::SerRon, nanoserde::DeRon))]
        #[cfg_attr(feature = "miniserde", derive(miniserde::Serialize, miniserde::Deserialize))]
        #[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(usize)]
        pub enum Locale {
            #[default]
            $($variant),+
        }

        impl Locale {
            pub const COUNT: usize = [$(stringify!($variant)),+].len();
            pub const VARIANTS: [Self; Self::COUNT] = [$(Self::$variant),+];
            pub const LABELS: [&'static str; Self::COUNT] = [$($label),+];
            localize_it::__init_locale_default_const!($($variant),+);

            #[inline]
            pub fn iter() -> impl Iterator<Item = (Self, &'static str)> {
                Self::iter_variants().zip(Self::iter_labels())
            }

            #[inline]
            pub fn iter_variants() -> impl Iterator<Item = Self> {
                Self::VARIANTS.into_iter()
            }

            #[inline]
            pub fn iter_labels() -> impl Iterator<Item = &'static str> {
                Self::LABELS.into_iter()
            }

            #[inline]
            pub const fn to_usize(self) -> usize {
                self as usize
            }

            #[inline]
            pub const fn from_usize(value: usize) -> Option<Self> {
                match value {
                    $(
                        _ if value == Self::$variant.to_usize() => Some(Self::$variant),
                    )+
                    _ => None,
                }
            }

            #[inline]
            pub fn from_usize_or_default(value: usize) -> Self {
                Self::from_usize(value).unwrap_or_default()
            }

            #[inline]
            pub const fn to_str(self) -> &'static str {
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

            #[inline]
            pub fn from_str_caseless(str: &str) -> Option<Self> {
                match str {
                    $(
                        _ if str.eq_ignore_ascii_case(stringify!($variant)) => Some(Self::$variant),
                    )+
                    _ => None,
                }
            }

            #[inline]
            pub fn from_str_caseless_or_default(str: &str) -> Self {
                Self::from_str_caseless(str).unwrap_or_default()
            }
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
