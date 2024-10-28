

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, Signature, Token, Type};


#[proc_macro_attribute]
pub fn received(args: TokenStream, item: TokenStream) -> TokenStream {

    let args = parse_macro_input!(args with Punctuated<Type, Token![,]>::parse_terminated);

    
    let ty = args.get(0).expect("No arguments");

    let ItemFn {
        sig,
        vis,
        block,
        ..
    } = parse_macro_input!(item as ItemFn);

    
    let trait_name = sig.ident.clone();

    quote!{
        #[allow(non_camel_case_types)]
        #vis trait #trait_name {
            #sig;
        }

        impl #trait_name for #ty {
            #sig 
            #block
        }
    }.into()
}

#[proc_macro_attribute]
pub fn closure(_: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        sig: Signature{
            ident,
            inputs,
            output,
            ..
        },
        block,
        ..
    } = parse_macro_input!(item as ItemFn);

    let inputs = inputs.iter().map(|i| quote!(#i));

    quote!{
        #(#attrs)*
        let #ident = |#(#inputs),*| #output #block ;
    }.into()
}