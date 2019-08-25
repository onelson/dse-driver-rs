extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse::{Parse, ParseBuffer};

#[proc_macro_derive(Ptr, attributes(ptr_type))]
pub fn ptr_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ptr(&ast)
}

#[proc_macro_derive(PtrMut, attributes(ptr_type))]
pub fn ptr_mut_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ptr_mut(&ast)
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

fn extract_ptr_kind(attrs: &[syn::Attribute]) -> syn::Ident {
    let attribute = attrs
        .iter()
        .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "ptr_type")
        .nth(0)
        .expect("ptr_type attribute required for deriving PtrProxy.");

    let PtrKind(ptr_kind): PtrKind =
        syn::parse2(attribute.clone().tokens).expect("Invalid ptr_type attribute.");

    ptr_kind
}

fn impl_ptr(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let ptr_kind = extract_ptr_kind(&ast.attrs);

    let gen = quote! {
        impl Ptr for #name {
            type T = #ptr_kind;
            fn ptr(&self) -> *const Self::T { self._ptr }
        }
    };
    gen.into()
}

fn impl_ptr_mut(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let ptr_kind = extract_ptr_kind(&ast.attrs);

    let gen = quote! {
        impl PtrMut for #name {
            type T = #ptr_kind;
            fn ptr_mut(&mut self) -> *mut Self::T { self._ptr }
        }
    };
    gen.into()
}
