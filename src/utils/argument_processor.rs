use crate::utils::aliases::{LocalizeItError, LocalizeItResult};
use proc_macro2::Ident;
use std::collections::HashSet;

pub struct ArgumentProcessor {
    pub exist_arguments: HashSet<String>,
}

impl ArgumentProcessor {
    pub fn new() -> Self {
        Self {
            exist_arguments: HashSet::new(),
        }
    }

    pub fn process(&mut self, name: &Ident) -> LocalizeItResult<String> {
        let name_string = name.to_string();

        if self.exist_arguments.insert(name_string.clone()) {
            Ok(name_string)
        } else {
            Err(LocalizeItError::DuplicateArgument(name.clone()))
        }
    }
}
