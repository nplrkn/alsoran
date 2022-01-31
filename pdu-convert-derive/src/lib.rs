use quote::quote;
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(PduConvert)]
pub fn pdu_convert_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let name = &ast.ident;
    let gen = quote! {
        impl From<#name> for F1apPdu {
            fn from(x: #name) -> Self {
                F1apPdu::#name(x)
            }
        }
    };
    gen.into()
}
