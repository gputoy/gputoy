mod doc_comment;
mod method;
mod ts;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn gpu_method(_args: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemFn);
    let method = match method::parse_macro_item(item) {
        Ok(method) => method,
        Err(e) => return e,
    };
    method::generate_tokens(method)
}

#[proc_macro]
pub fn gen_ts(tokens: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(tokens as ts::TsSnippet);
    ts::generate_tokens(item)
}

#[proc_macro]
pub fn extract_doc_comment(tokens: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(tokens as doc_comment::AttributeList);
    doc_comment::extract_doc_comment(item)
}
