use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Block, Ident, ItemFn, Signature, Type, TypePath};

#[derive(Debug, Clone)]
pub struct MethodItem {
    pub(crate) ident: Ident,
    pub(crate) inputs: Vec<MethodInput>,
    pub(crate) block: Block,
    pub(crate) output: Option<Type>,
    pub(crate) is_output_result: bool,
}

#[derive(Debug, Clone)]
pub struct MethodInput {
    pub(crate) ident: Ident,
    pub(crate) ty_path: TypePath,
    pub(crate) preserve_type: bool,
}

pub fn parse_macro_item(item: ItemFn) -> Result<MethodItem, TokenStream> {
    let ItemFn { block, sig, .. } = item;

    let block = *block;
    let Signature {
        ident,
        inputs,
        output,
        ..
    } = sig;

    let ident = format_ident!("__{ident}");
    let output = if let syn::ReturnType::Type(_, output) = output {
        Some(*output)
    } else {
        None
    };

    let is_output_result = if let Some(ref output_ty) = output {
        let Type::Path(ty_path) = output_ty else {
                return Err(compile_error(
                    output_ty,
                    "only named types are supported for method output",
                ))
        };

        ty_path
            .path
            .segments
            .iter()
            .any(|segment| segment.ident == "Result")
    } else {
        false
    };

    let inputs = {
        let mut _inputs = Vec::with_capacity(inputs.len());
        for input in inputs {
            let syn::FnArg::Typed(ty) = input else {
            return Err(
                compile_error(input, "'self' arguments not supported")
            )
        };

            let syn::Type::Path(ty_path) = *ty.ty else {
            return Err(compile_error(*ty.ty, "only named types are supported"))
        };
            let syn::Pat::Ident(ident) = *ty.pat else {
            return Err(
                compile_error(*ty.pat, "only named types are supported")
            )
        };
            _inputs.push(MethodInput {
                preserve_type: is_wasm_abi_type(&ty_path),
                ident: ident.ident,
                ty_path,
            });
        }
        _inputs
    };

    let _ident_name = ident.to_string();

    Ok(MethodItem {
        ident,
        inputs,
        output,
        block,
        is_output_result,
    })
}

pub fn generate_tokens(method: MethodItem) -> TokenStream {
    let MethodItem {
        ident,
        inputs,
        block,
        output,
        is_output_result,
    } = method;

    let input_decl = inputs
        .clone()
        .into_iter()
        .map(|input| {
            let MethodInput {
                ident,
                ty_path,
                preserve_type,
            } = input;
            if preserve_type {
                quote! {
                    #ident: #ty_path,
                }
            } else {
                quote! {
                    #ident: ::wasm_bindgen::JsValue,
                }
            }
        })
        .collect::<Vec<_>>();

    let input_parse = inputs
        .into_iter()
        .filter(|input| !input.preserve_type)
        .map(|input| {
            let MethodInput { ident, ty_path, .. } = input;
            quote! {
                let #ident: #ty_path = ::serde_wasm_bindgen::from_value(#ident)
                    .map_err(gpu_common::method::MethodError);
            }
        })
        .collect::<Vec<_>>();

    let return_tail = if is_output_result {
        quote! {
            match result {
                Ok(res) => ::serde_wasm_bindgen::to_value(&res)
                    .map_err(gpu_common::method::MethodError::SerdeWasmBindgen),
                Err(e) => Err(
                    gpu_common::method::MethodError::Method(
                        gpu_common::IntoClientError::into_client_error(e)
                    )),
            }
        }
    } else {
        quote! {
            ::serde_wasm_bindgen::to_value(&result)
                .map_err(gpu_common::method::MethodError::SerdeWasmBindgen)
        }
    };

    let tokens = quote! {
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn #ident(
            #(
                #input_decl
            )*
        ) -> ::core::result::Result<::wasm_bindgen::JsValue, gpu_common::method::MethodError> {
            #(
                #input_parse
            )*
            let result: #output = {
                #block
            };
            #return_tail
        }
    };
    tokens.into()
}

fn is_wasm_abi_type(path: &TypePath) -> bool {
    path.path
        .get_ident()
        .map(|ident| {
            matches!(
                ident.to_string().as_str(),
                "str"
                    | "String"
                    | "char"
                    | "u8"
                    | "u16"
                    | "u32"
                    | "u64"
                    | "usize"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "isize"
                    | "f32"
                    | "f64"
            )
        })
        .unwrap_or(false)
}

pub fn compile_error<T, U>(tokens: T, message: U) -> TokenStream
where
    T: quote::ToTokens,
    U: std::fmt::Display,
{
    syn::Error::new_spanned(tokens, message)
        .into_compile_error()
        .into()
}
