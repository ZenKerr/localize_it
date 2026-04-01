use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream}, Error, Expr, LitStr,
    Token,
};

pub enum Item {
    Variant { name: Ident, label: LitStr },
    Optional { name: String, value: Expr },
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let name = input.parse::<Ident>()?;

        if input.peek(Token![=]) && !input.peek(Token![=>]) {
            input.parse::<Token![=]>()?;

            let name = name.to_string();
            let value = input.parse()?;

            Ok(Self::Optional { name, value })
        } else {
            let label = if input.peek(Token![=>]) {
                input.parse::<Token![=>]>()?;

                input.parse()?
            } else {
                LitStr::new(&name.to_string(), name.span())
            };

            Ok(Self::Variant { name, label })
        }
    }
}
