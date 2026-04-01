pub mod item;

use item::Item;
use std::vec::IntoIter;
use syn::{
    parse::{Parse, ParseStream}, Error,
    Token,
};

pub struct Input {
    items: Vec<Item>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let mut arguments = Vec::new();

        let mut named_started = false;
        while !input.is_empty() {
            let argument = input.parse()?;

            match &argument {
                Item::Variant { .. } => {
                    if named_started {
                        return Err(Error::new(
                            input.span(),
                            "Expected variants list to precede optional arguments",
                        ));
                    }
                }
                Item::Optional { .. } => {
                    named_started = true;
                }
            }

            arguments.push(argument);

            if input.is_empty() {
                break;
            } else {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { items: arguments })
    }
}

impl IntoIterator for Input {
    type Item = Item;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
