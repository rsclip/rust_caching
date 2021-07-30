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
    let cache_hs: HashSet<Ident> = parse_macro_input!(metadata as Args).vars;

    // Get a reference to the cache_struct Ident (only value in cache_hs)
    let cache_ident: &Ident = match cache_hs.iter().next() {
        Some(x) => { x },
        None => { panic!("no cache argument") }
    };

    let input_fn: Item = syn::parse(item).expect("failed to parse input");

    // Ensure input_fn is a function
    match input_fn {
        Item::Fn(ref fn_item) => {
            // Is a function, get the arguments
            let argument_id = encode_arguments(fn_item);
        },
        
        _ => {
            println!("not a function");
        }
    }

    // Add code to accomodate for cache


    // Convert input_fn (syn::Item) back into TokenStream
    // and return it
    quote!(#input_fn).into()
}

fn encode_arguments(struct_: &syn::ItemFn) -> u64 {
    1u64
}

macro_rules! args_id { 
    ($($x:expr), *) => {{
        let mut s = DefaultHasher::new();
        $(
            $x.hash(&mut s);
        )*

        s.finish()
    };}
}