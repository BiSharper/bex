use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(LexicalScope)]
pub fn proc_lexer_scope_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let meta_type = ast.ident;

    return quote::quote! {
        impl bex::LexicalScope for #meta_type {

        }
    }.into()
}