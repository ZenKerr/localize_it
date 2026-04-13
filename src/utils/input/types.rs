use syn::{Error, Expr};

pub type ParseFunction<T> = fn(&str, &Expr) -> Result<T, Error>;
