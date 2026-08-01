pub use crate::utils::error::Error as LocalizeItError;
pub use proc_macro_crate::Error as ProcMacroCrateError;
pub use syn::{Error as SynError, Result as SynResult};

pub type LocalizeItResult<T> = Result<T, LocalizeItError>;
