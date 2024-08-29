use mangler::mangle;
use quote::quote;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn cxx_link(attr: TokenStream, token_stream: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(token_stream as syn::ForeignItemFn);
    let attr = syn::parse_macro_input!(attr as syn::LitStr);
    let mangled = mangle(attr.value());

    quote! {
        #[link_name = #mangled]
        #func
    }.into()
}
