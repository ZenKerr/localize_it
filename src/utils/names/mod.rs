pub mod provider;

pub const ENUM_LOCALE: &str = "Locale";
pub const MOD_STORAGE: &str = "storage";
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
pub const MACRO_LOCALIZE: &str = if cfg!(feature = "short_names") {
    "l"
} else {
    "localize"
};
pub const MACRO_EXPRESSION_PART: &str = if cfg!(feature = "short_names") {
    "e_p"
} else {
    "expression_part"
};
pub const MACRO_EXPRESSIONS_PART: &str = if cfg!(feature = "short_names") {
    "es_p"
} else {
    "expressions_part"
};
pub const MACRO_EXPRESSIONS_FROM_FILES: &str = if cfg!(feature = "short_names") {
    "es_f"
} else {
    "expressions_from_files"
};
