use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, Expr, Ident, LitStr, Token};

#[derive(Debug)]
pub struct TsSnippet {
    lines: Vec<LineGroup>,
}

#[derive(Debug)]
enum LineGroup {
    Single(LitStr),
    Multi {
        value: LitStr,
        val_ident: Ident,
        iter_expr: Expr,
    },
}

impl Parse for TsSnippet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut lines = Vec::new();
        loop {
            if let Ok(literal) = input.parse::<LitStr>() {
                lines.push(LineGroup::Single(literal));
            } else if input.parse::<Token![for]>().is_ok() {
                let val_ident = input.parse::<Ident>()?;
                input.parse::<Token![in]>()?;
                let iter_expr = input.parse::<Expr>()?;
                let literal = input.parse::<LitStr>()?;
                lines.push(LineGroup::Multi {
                    val_ident,
                    iter_expr,
                    value: literal,
                });
            } else {
                break;
            }
        }
        Ok(TsSnippet { lines })
    }
}

pub fn generate_tokens(item: TsSnippet) -> TokenStream {
    let lines = item.lines.iter().map(|line| match line {
        LineGroup::Single(value) => quote! {
            _buf.push(format!(#value));
        },
        LineGroup::Multi {
            value,
            val_ident,
            iter_expr,
        } => quote! {
            #iter_expr.for_each(|#val_ident| _buf.push(format!(#value)));
        },
    });

    quote! {
        {
            let mut _buf = ::std::vec::Vec::new();
            #(#lines)*
            _buf.join("\n")
        }
    }
    .into()
}
