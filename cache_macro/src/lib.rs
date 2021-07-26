extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
/// attr: argument passed into (i.e a struct)
/// item: actual function (i.e "fn test() { }")
pub fn test_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {}, item: {} ---------------------------------------------------------------------", attr.to_string(), item.to_string());
    item
}
