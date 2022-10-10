use proc_macro::TokenStream;

#[proc_macro_derive(ToString)]
pub fn derive(input: TokenStream) -> TokenStream {
    // マクロの実装を呼び出すだけ
    // proc_macro から proc_macro2 の TokenStream に変換する
    tostring_macro_internals::derive(input.into()).into()
}
