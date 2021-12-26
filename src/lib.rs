//! At the time of writing, [`syn`] crate doesn't offer a way to parse [outer attributes][outer attribute] (for example,
//! `#[repr(c)]`) without declaring a struct and reinventing a wheel by implementing [`syn::parse::Parse`] trait. Unlike [`syn::Attribute`],
//! depending on the enabled feature `same_layout` or `different_layout` the chosen
//! [`different_layout::OuterAttribute`] or [`same_layout::OuterAttribute`], respectively, will have
//! [`syn::parse::Parse`] available.
//!
//! # Example
//!
//! ```rust
//! extern crate syn;
//! extern crate outer_attribute;
//!
//! use outer_attribute::different_layout::OuterAttribute;
//!
//! fn main() {
//!     assert!(matches!(syn::parse_str::<OuterAttribute>("#[repr(C)]"), Ok(_)));
//!     assert!(matches!(syn::parse_str::<OuterAttribute>("Not an outer attribute"), Err(_)));
//! }
//! ```
//!
//! The default feature is `different_layout` and opting for `same_layout` instead can be considered when
//! there is functionality that is available on [`syn::Attribute`] but not on either `OuterAttribute`.
//!
//! [outer attribute]: https://doc.rust-lang.org/reference/attributes.html

// Restriction to only one feature can be relaxed if there is a need for that in the future.
// However, using both layouts is very wasteful.
#[cfg(not(any(feature = "same_layout", feature = "different_layout")))]
compile_error!("At least one of the features \"same_layout\" or \"different_layout\" must be enabled for this crate.");
#[cfg(all(feature = "same_layout", feature = "different_layout"))]
compile_error!("Exactly one of the features \"same_layout\" or \"different_layout\" must be enabled for this crate.");

#[cfg(any(doc, feature = "different_layout"))]
pub mod different_layout {
    /// As opposed to [`same_layout::OuterAttribute`], [`different_layout::OuterAttribute`] takes up less space because
    /// of the omitted `style`: [`syn::AttrStyle`] field.
    ///
    /// [`same_layout::OuterAttribute`]: crate::same_layout::OuterAttribute
    /// [`different_layout::OuterAttribute`]: crate::different_layout::OuterAttribute
    pub struct OuterAttribute {
        pub pound_token: syn::Token![#],
        pub bracket_token: syn::token::Bracket,
        pub path: syn::Path,
        pub tokens: proc_macro2::TokenStream,
    }

    impl syn::parse::Parse for OuterAttribute {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let content;
            Ok(OuterAttribute {
                pound_token: input.parse()?,
                bracket_token: syn::bracketed!(content in input),
                path: content.parse()?,
                tokens: content.parse()?,
            })
        }
    }
}

#[cfg(any(doc, feature = "same_layout"))]
pub mod same_layout {
    /// As opposed to [`different_layout::OuterAttribute`], [`crate::same_layout::OuterAttribute`] takes up more space
    /// because of the `style`: [`syn::AttrStyle`] field. In return, however, you get to use any functions
    /// that are implemented on [`syn::Attribute`] and related types.
    ///
    /// [`different_layout::OuterAttribute`]: crate::different_layout::OuterAttribute
    /// [`same_layout::OuterAttribute`]: crate::same_layout::OuterAttribute
    pub struct OuterAttribute(pub syn::Attribute);

    // TODO: Create macro: `copypaste!("syn/attr.rs","1.0.84","521-530")`
    // Copy-pasted from https://docs.rs/syn/1.0.84/src/syn/attr.rs.html#521-530
    fn single_parse_outer(input: syn::parse::ParseStream) -> syn::Result<syn::Attribute> {
        let content;
        Ok(syn::Attribute {
            pound_token: input.parse()?,
            style: syn::AttrStyle::Outer,
            bracket_token: syn::bracketed!(content in input),
            path: content.call(syn::Path::parse_mod_style)?,
            tokens: content.parse()?,
        })
    }

    impl syn::parse::Parse for OuterAttribute {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(OuterAttribute(
                input.call::<syn::Attribute>(single_parse_outer)?,
            ))
        }
    }
}
