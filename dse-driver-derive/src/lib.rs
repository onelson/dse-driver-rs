extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse::{Parse, ParseBuffer};

#[proc_macro_derive(PtrProxy, attributes(ptr_type))]
pub fn ptr_proxy_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ptr_proxy(&ast)
}

struct PtrKind(syn::Ident);

impl Parse for PtrKind {
    fn parse(input: &ParseBuffer) -> syn::parse::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let kind = content.parse()?;
        Ok(PtrKind(kind))
    }
}

fn impl_ptr_proxy(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let attribute = ast
        .attrs
        .iter()
        .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "ptr_type")
        .nth(0)
        .expect("ptr_type attribute required for deriving PtrProxy.");

    let PtrKind(ptr_kind): PtrKind =
        syn::parse2(attribute.clone().tokens).expect("Invalid ptr_type attribute.");
    let gen = quote! {
        impl PtrProxy for #name {
            type T = #ptr_kind;
            fn ptr(&self) -> *const Self::T { self._ptr }
            fn ptr_mut(&mut self) -> *mut Self::T { self._ptr }
        }
    };
    gen.into()
}
