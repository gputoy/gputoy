extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Lit};

pub struct AttributeList(Vec<Attribute>);

impl Parse for AttributeList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(AttributeList(input.call(syn::Attribute::parse_outer)?))
    }
}

pub fn extract_doc_comment(item: AttributeList) -> TokenStream {
    let doc_comments: Vec<String> = item
        .0
        .iter()
        .filter_map(|attr| {
            if let Ok(meta) = attr.parse_meta() {
                if meta.path().is_ident("doc") {
                    if let syn::Meta::NameValue(name_value) = meta {
                        if let Lit::Str(lit_str) = name_value.lit {
                            return Some(lit_str.value());
                        }
                    }
                }
            }
            None
        })
        .collect();

    let doc_comment_string = doc_comments.join("\n");
    quote! {
        #doc_comment_string
    }
    .into()
}
