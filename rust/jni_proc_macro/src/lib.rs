use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Nothing, Result};
use syn::token::Type;
use syn::{parse_quote, Attribute, FnArg, Ident, ItemFn, PatType, ReturnType, parse_macro_input, ItemType};

#[proc_macro_attribute]
pub fn jni_func(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = TokenStream2::from(args);
    let input = TokenStream2::from(input);
    
    let expanded = match parse(args, input.clone()) {
        Ok(function) => expand_jni_func(function),
        Err(parse_error) => {
            let compile_error = parse_error.to_compile_error();
            quote!(#compile_error #input)
        }
    };
    TokenStream::from(expanded)
}

fn parse(args: TokenStream2, input: TokenStream2) -> Result<ItemFn> {
    let function: ItemFn = syn::parse2(input)?;
    let _: Nothing = syn::parse2::<Nothing>(args)?;
    Ok(function)
}

fn expand_jni_func(mut function: ItemFn) -> TokenStream2 {
    let sig = function.sig;
    let args = sig.inputs;
    let name = sig.ident;
    let block = *function.block;
    let func_name = format!("Java{}", name).parse().unwrap();
    let out = ItemType sig.output;
    quote! {
        pub extern "system" fn #func_name (#args) -> #ret_type {
            if let Err(err) = #block {
                env.throw(err).unwrap();
            }
        }
    }
}