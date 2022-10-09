use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
    // parse_macro_input! は proc_macro2 配下の TokenStream には使えないので
    // syn::parse2(input) でパースする必要がある
    let input: DeriveInput = syn::parse2(input).unwrap();

    let src_ident = input.ident;
    let src_ident_str = src_ident.to_string();

    (quote! {
        impl ::std::fmt::Debug for #src_ident {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.debug_struct( #src_ident_str )
                    .finish()
            }
        }
    }).into()
}
