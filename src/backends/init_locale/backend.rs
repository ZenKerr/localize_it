use crate::{
    backends::init_locale::{
        arguments::Arguments,
        parts::{
            enum_locale, macro_expression, macro_expression_part, macro_expressions,
            macro_expressions_from_files, macro_expressions_part, macro_localize, mod_storage,
        },
    },
    utils::NamesProvider,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Result};

pub fn backend(input: TokenStream) -> Result<TokenStream> {
    let arguments = &parse::<Arguments>(input)?;
    let names_provider = &NamesProvider::new(arguments.path.clone());

    let locale = enum_locale(arguments, names_provider);
    let storage = mod_storage(arguments, names_provider);
    let expression = macro_expression(arguments, names_provider)?;
    let expressions = macro_expressions(names_provider);
    let localize = macro_localize(arguments, names_provider);
    let expression_part = macro_expression_part(names_provider);
    let expressions_part = macro_expressions_part(names_provider);
    let expressions_from_files = macro_expressions_from_files(arguments, names_provider)?;

    Ok(quote! {
        #locale
        #storage
        #expression
        #expressions
        #localize
        #expression_part
        #expressions_part
        #expressions_from_files
    }
    .into())
}
