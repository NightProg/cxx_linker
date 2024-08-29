//! A procedural macro for linking Rust functions with C++ using Itanium name mangling.
//! # Example
//! A procedural macro for linking Rust functions with C++ using Itanium name mangling.
//! # Example
//! ```no_run
//! use cxx_linker::cxx_link;
//!
//! extern "C" {
//!     #[cxx_link("blobstore::foo<int>::bar(int, blobstore::SomeType&, bool)")]
//!     fn bar(a: i32, b: *mut (), c: bool);
//! }
//! ```
//! will be expanded to
//! ```no_run
//! extern "C" {
//!     #[link_name = "_ZN9blobstore3fooIiE3barEiRNS_8SomeTypeEb"]
//!     fn bar(a: i32, b: *mut (), c: bool);
//! }
//! ```

use mangler::mangle;
use quote::quote;
use proc_macro::TokenStream;

/// A procedural macro for linking Rust functions with C++ using Itanium name mangling.
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
