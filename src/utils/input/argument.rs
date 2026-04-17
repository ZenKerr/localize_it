use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream}, Error, Expr, LitStr,
    Token,
};

pub enum Argument {
    Mapped { name: Ident, label: LitStr },
    Named { name: Ident, value: Expr },
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let name = input.parse::<Ident>()?;

        if input.peek(Token![=]) && !input.peek(Token![=>]) {
            input.parse::<Token![=]>()?;

            let value = input.parse()?;

            Ok(Self::Named { name, value })
        } else {
            let label = if input.peek(Token![=>]) {
                input.parse::<Token![=>]>()?;

                input.parse()?
            } else {
                LitStr::new(&name.to_string(), name.span())
            };

            Ok(Self::Mapped { name, label })
        }
    }
}
