extern crate outer_attribute;
extern crate syn;

use outer_attribute::different_layout::OuterAttribute;

fn main() {
    assert!(matches!(
        syn::parse_str::<OuterAttribute>("#[repr(C)]"),
        Ok(_)
    ));
    assert!(matches!(
        syn::parse_str::<OuterAttribute>("Not an outer attribute"),
        Err(_)
    ));
}
