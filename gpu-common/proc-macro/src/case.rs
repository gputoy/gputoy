extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token};

use convert_case::Case;

pub struct Input {
    case: Case,
    value: String,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let case = input.parse::<LitStr>()?;
        let case = match_case(&case)?;
        input.parse::<Token![,]>()?;
        if let Ok(ident) = input.parse::<Ident>() {
            Ok(Self {
                case,
                value: ident.to_string(),
            })
        } else if let Ok(lit_str) = input.parse::<LitStr>() {
            Ok(Self {
                case,
                value: lit_str.value(),
            })
        } else {
            Err(syn::Error::new(
                input.span(),
                "Expected Ident or string literal",
            ))
        }
    }
}

pub fn generate_tokens(item: Input) -> TokenStream {
    let cased_str = convert_case::Casing::to_case(&item.value, item.case);
    quote! {
        #cased_str
    }
    .into()
}

fn match_case(lit: &LitStr) -> syn::Result<Case> {
    match lit.value().as_str() {
        "camelCase" => Ok(Case::Camel),
        "kebab-case" => Ok(Case::Kebab),
        "lower case" => Ok(Case::Lower),
        "PascalCase" => Ok(Case::Pascal),
        "UPPER_SNAKE_CASE" => Ok(Case::UpperSnake),
        "snake_case" => Ok(Case::Snake),
        "Title Case" => Ok(Case::Title),
        _ => Err(syn::Error::new(
            lit.span(),
            format!("{} not supported", lit.value()),
        )),
    }
}
