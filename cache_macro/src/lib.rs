extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::{TokenStream};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, Ident, Item, ItemFn};
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

    let input_fn: ItemFn = syn::parse(item).expect("failed to parse input");

    // Split the code into function declaration and inner code function
    let (fn_declaration, inner_code) = split_function_code(&input_fn);

    // Add code to accomodate for cache
    let new_fn = quote! {
        #fn_declaration {
            match #cache_ident.check_cache() {
                std::option::Option::Some(cached_result) => { cached_result }.
                std::option::Option::None = #inner_code
            }
        }
    };

    println!("{}", new_fn.to_string());

    // Return the new function
    new_fn.into()
    // quote!(#input_fn).into()
}

fn split_function_code(input_fn: &ItemFn) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ts_fn = quote!(#input_fn.block); // tokenstream for function

    let mut fn_iter = ts_fn.into_iter();

    // Get the first 3 elements (function declaration)
    /*
    
    -- PROBLEM --
    Because quote! parses the String object (not &str), it
    includes the quotation marks in the final code.
    You could try taking 3 via iter.take(3) but idk good luck


    */
    let mut declaration_str = String::new();

    for _ in 0..3 {
        declaration_str.push_str(&fn_iter.next().unwrap().to_string());
        declaration_str.push(' ');  
    }

    let declaration = quote!(#declaration_str);

    // Next element is the entire function body
    let body = fn_iter.next().unwrap();
    let body = quote!(#body);

    (declaration, body)
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