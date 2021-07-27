extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, Ident, Item};
use quote::{quote};

use std::collections::HashSet;

#[derive(Debug)]
struct Args {
    vars: HashSet<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated(input)?;

        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

#[proc_macro_attribute]
/// metadata: argument passed into (i.e a struct)
/// item: actual function (i.e "fn test() { }")
pub fn test_macro(metadata: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the caching struct - convert from TokenStream to an Item
    // or a TokenStream to an Args struct
    // let cache_struct: syn::Item = syn::parse(metadata).expect("failed to parse");
    let cache_struct = parse_macro_input!(metadata as Args);
    let input_fn: Item = syn::parse(item).expect("failed to parse input");

    // Ensure input_fn is a function
    match input_fn {
        Item::Fn(ref fn_item) => {
            // Is a function, get the arguments

        },
        
        _ => {
            println!("not a function");
        }
    }

    // Convert input_fn (syn::Item) back into TokenStream
    // and return it
    quote!(#input_fn).into()
}

macro_rules! args_id { 
    ($($x:expr), *) => {{
        let mut s = DefaultHasher::new();
        let mut v: Vec<u64> = Vec::new();
        $(
            $x.hash(&mut s);
        )*

        s.finish()
    };}
}