use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AutoJIntoResponse)]
pub fn derive_json_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        #[cfg(feature = "server")]
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                axum::Json(self).into_response()
            }
        }
    };

    TokenStream::from(expanded)
}