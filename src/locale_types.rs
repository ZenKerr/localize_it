/// Defines types associated with [`Locale`](crate::init_locale!).
///
/// This trait is automatically implemented when [`Locale`](crate::init_locale!) is initialized.
/// It is used to determine the type of localized expressions.
pub trait LocaleTypes {
    /// Represents a localized expression.
    ///
    /// The concrete type is `[&'static str; Locale::COUNT]`.
    type Expression;
}
