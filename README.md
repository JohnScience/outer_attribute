# A library providiving `OuterAttribute` that implements `syn::parse::Parse` trait.

At the time of writing, [`syn`] crate doesn't offer a way to parse [outer attributes][outer attribute] (for example, 
`#[repr(C)]`) without declaring a struct and reinventing a wheel by implementing [`syn::parse::Parse`] trait. Unlike [`syn::Attribute`],
depending on the enabled feature `same_layout` or `different_layout` the chosen
[`different_layout::OuterAttribute`] or [`same_layout::OuterAttribute`], respectively, will have
[`syn::parse::Parse`] available.

# Example

```rust
extern crate syn;
extern crate outer_attribute;

use outer_attribute::different_layout::OuterAttribute;

fn main() {
    assert!(matches!(syn::parse_str::<OuterAttribute>("#[repr(C)]"), Ok(_)));
    assert!(matches!(syn::parse_str::<OuterAttribute>("Not an outer attribute"), Err(_)));
}
```

The default feature is `different_layout` and opting for `same_layout` instead can be considered when
there is functionality that is available on [`syn::Attribute`] but not on either `OuterAttribute`.

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[outer attribute]: https://doc.rust-lang.org/reference/attributes.html
[`syn`]: https://docs.rs/syn/latest/syn/
[`syn::parse::Parse`]: https://docs.rs/syn/latest/syn/parse/trait.Parse.html
[`syn::Attribute`]: https://docs.rs/syn/latest/syn/struct.Attribute.html
[`different_layout::OuterAttribute`]: https://docs.rs/outer_attribute/latest/outer_attribute/different_layout/struct.OuterAttribute.html
[`same_layout::OuterAttribute`]: https://docs.rs/outer_attribute/latest/outer_attribute/same_layout/struct.OuterAttribute.html