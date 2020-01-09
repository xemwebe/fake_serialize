///! A rust macro for implementing dummy implementations of the traits
///!  `serde::Serialize` and `serde::Deserialize`
///!
///! Sometimes, you run in the problem to use some trait or function from an external crate, that is
///! constraint to types that implement the traits `serde::Serialize` or/and `serde::Deserialize`, even if 
///! this is not an requirement for all use cases. Here, using a dummy implementation that just returns
///! an error if `serialize` or `deserialize` is calls comes handy.
///!
///! This crate provides a derive macro that just provides these dummy implementations, e.g.
///! ```rust
///! use fake_serialize::{FakeSerialize,FakeDeserialize};
///!
///! #[derive(FakeSerialize,FakeDeserialize)]
///! struct SomeStruct {
///!     ...
///! }
///! ```


extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(FakeSerialize)]
/// Implement fake serialize for given type
pub fn fake_serialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_fake_serialize_macro(&ast)
}

#[proc_macro_derive(FakeDeserialize)]
/// Implement fake deserialize for given type
pub fn fake_deserialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_fake_deserialize_macro(&ast)
}

/// Implementation details of fake serialize
fn impl_fake_serialize_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl serde::Serialize for #name {
            fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer
                {
                    Err(serde::ser::Error::custom(format!("serialization is disabled for this type")))
                }
            }
    };
    gen.into()
}

/// Implementation details of fake deserialize
fn impl_fake_deserialize_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>
                {
                    Err(serde::de::Error::custom(format!("deserialization is disabled for this type")))
                }
            }

    };
    gen.into()
}
