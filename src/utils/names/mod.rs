pub mod provider;
mod utils;

use crate::utils::names::utils::short_or_default;

pub const ENUM_LOCALE: &str = "Locale";
pub const MOD_STORAGE: &str = "storage";
pub const MACRO_EXPRESSION: &str = short_or_default("e", "expression");
pub const MACRO_EXPRESSIONS: &str = short_or_default("es", "expressions");
pub const MACRO_LOCALIZE: &str = short_or_default("l", "localize");
pub const MACRO_EXPRESSION_PART: &str = short_or_default("e_p", "expression_part");
pub const MACRO_EXPRESSIONS_PART: &str = short_or_default("es_p", "expressions_part");
pub const MACRO_EXPRESSIONS_FROM_FILES: &str = short_or_default("es_f", "expressions_from_files");
