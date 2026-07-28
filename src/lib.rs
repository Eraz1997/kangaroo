mod add_fields;

use add_fields::add_fields_impl;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn add_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
    add_fields_impl(attr.into(), item.into()).into()
}
