use crate::{
    arguments::Arguments,
    names::Names,
    parts::{enum_locale, macro_expression, macro_expressions, macro_localize, mod_storage},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Error};

pub fn backend(input: TokenStream) -> Result<TokenStream, Error> {
    let input = parse(input)?;
    let arguments = &Arguments::new(input)?;
    let names = &Names::new(arguments);

    let locale = enum_locale(arguments, names);
    let storage = mod_storage(arguments, names);
    let expression = macro_expression(names);
    let expressions = macro_expressions(names);
    let localize = macro_localize(arguments, names);

    Ok(quote! {
        #locale
        #storage
        #expression
        #expressions
        #localize
    }
    .into())
}
