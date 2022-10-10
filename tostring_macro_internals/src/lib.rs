use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};

fn error<D: std::fmt::Display>(span: proc_macro2::Span, msg: D) -> TokenStream {
    syn::Error::new(span, msg).to_compile_error()
}

pub fn derive(input: TokenStream) -> TokenStream {
    // parse_macro_input! は proc_macro2 配下の TokenStream には使えないので
    // syn::parse2(input) でパースする必要がある
    let input: DeriveInput = syn::parse2(input).unwrap();

    let src_fields;
    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        src_fields = fields;
    } else {
        return error(input.ident.span(), "Currently you can just derive CustomDebug on structs").into();
    }

    let src_ident = input.ident;
    let src_ident_str = src_ident.to_string();

    let formatter_fn = match &src_fields {
        Fields::Named(_) => {
            quote! { debug_struct( #src_ident_str ) }
        }
        Fields::Unnamed(_) => {
            quote! { debug_tuple( #src_ident_str ) }
        }
        Fields::Unit => {
            quote! { debug_struct( #src_ident_str ) }
        }
    };

    let mut formatter_field_args = vec![];
    let pattern = "{:?}";

    for (i, field) in src_fields.iter().enumerate() {
        let field_ident = &field.ident;

        if let Some(ident) = field_ident {
            let ident_str = (*ident).to_string();
            formatter_field_args.push(quote! { #ident_str, &format_args!( #pattern , &self.#ident ) });
        } else {
            let i = proc_macro2::Literal::usize_unsuffixed(i);
            formatter_field_args.push(quote! { &self.#i });
        }
    }

    (quote! {
        impl ::std::fmt::Debug for #src_ident {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.#formatter_fn
                    #(  .field(#formatter_field_args)   )*
                    .finish()
            }
        }
    }).into()
}
