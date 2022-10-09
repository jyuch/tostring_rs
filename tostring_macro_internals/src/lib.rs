use proc_macro2::TokenStream;
use quote::quote;

pub fn derive(_input: TokenStream) -> TokenStream {
    (quote! {
        impl ::std::fmt::Debug for Struct {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("Piyo")
            }
        }
    }).into()
}
