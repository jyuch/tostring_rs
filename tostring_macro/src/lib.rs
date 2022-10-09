use proc_macro::TokenStream;

#[proc_macro_derive(ToString)]
pub fn derive(input: TokenStream) -> TokenStream {
    tostring_macro_internals::derive(input.into()).into()
}
