pub mod provider;

pub const ENUM_LOCALE: &str = "Locale";
pub const MACRO_EXPRESSION: &str = if cfg!(feature = "short_names") {
    "e"
} else {
    "expression"
};
pub const MACRO_EXPRESSIONS: &str = if cfg!(feature = "short_names") {
    "es"
} else {
    "expressions"
};
pub const MACRO_EXPRESSIONS_FROM_FILES: &str = if cfg!(feature = "short_names") {
    "es_f"
} else {
    "expressions_from_files"
};
pub const MACRO_LOCALIZE: &str = if cfg!(feature = "short_names") {
    "l"
} else {
    "localize"
};
pub const MOD_STORAGE: &str = "storage";
