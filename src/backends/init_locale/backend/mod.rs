use crate::{
    backends::init_locale::{
        arguments::Arguments,
        parts::{
            enum_locale, macro_expression, macro_expressions, macro_expressions_from_files,
            macro_localize, mod_storage,
        },
    },
    utils::NamesProvider,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Error};

pub fn backend(input: TokenStream) -> Result<TokenStream, Error> {
    let input = parse(input)?;
    let arguments = &Arguments::new(input)?;
    let names_provider = &NamesProvider::new(arguments.path.clone());

    let locale = enum_locale(arguments, names_provider);
    let storage = mod_storage(arguments, names_provider);
    let expression = macro_expression(names_provider);
    let expressions = macro_expressions(names_provider);
    let localize = macro_localize(arguments, names_provider);
    let expressions_from_files = macro_expressions_from_files(arguments, names_provider)?;

    Ok(quote! {
        #locale
        #storage
        #expression
        #expressions
        #localize
        #expressions_from_files
    }
    .into())
}
