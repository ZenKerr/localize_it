use proc_macro2::Ident;
use std::collections::HashSet;
use syn::{Error, Result};

pub struct ArgumentProcessor {
    pub exist_arguments: HashSet<String>,
}

impl ArgumentProcessor {
    pub fn new() -> Self {
        Self {
            exist_arguments: HashSet::new(),
        }
    }

    pub fn process(&mut self, name: &Ident) -> Result<String> {
        let name_string = name.to_string();

        if self.exist_arguments.insert(name_string.clone()) {
            Ok(name_string)
        } else {
            Err(Error::new(
                name.span(),
                format!("Duplicate argument `{name_string}`"),
            ))
        }
    }
}
