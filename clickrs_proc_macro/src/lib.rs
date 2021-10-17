extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::{parse_macro_input, Attribute, FnArg, ItemFn, Pat};

fn build_structopt_block(
    command_args: TokenStream2,
    input_itemfn: ItemFn,
    argument_macro_attributes: HashMap<String, TokenStream2>,
) -> TokenStream2 {
    let inputs = input_itemfn.clone().sig.inputs;

    let mut members_block = quote! {};

    for main_arg in inputs.iter() {
        let main_arg_pat = if let FnArg::Typed(pat_type) = main_arg {
            Some(pat_type)
        } else {
            None
        }
        .unwrap();

        let arg_name_ident = if let Pat::Ident(pat_ident) = &*main_arg_pat.pat {
            Some(pat_ident)
        } else {
            None
        }
        .unwrap()
        .ident
        .clone();

        let formatted_arg_name = format!("{}", &arg_name_ident);

        let mut structopt_member_block = quote! {};
        if argument_macro_attributes.contains_key(&formatted_arg_name) {
            let macro_args = argument_macro_attributes.get(&formatted_arg_name).unwrap();
            structopt_member_block = quote! { #macro_args }
        }

        members_block = quote! {
            #members_block

            #[structopt(#structopt_member_block)]
            #main_arg,
        }
    }

    let structopt_block = quote! {
        use structopt::StructOpt;

        #[derive(Debug, StructOpt)]
        #[structopt(#command_args)]
        struct __CommandStruct {
            #members_block
        }
    };

    structopt_block
}

fn build_inner_main_args(input_itemfn: ItemFn) -> TokenStream2 {
    let mut inner_main_args = quote! {};
    let inputs = input_itemfn.clone().sig.inputs;

    for main_arg in inputs.iter() {
        let main_arg_pat = if let FnArg::Typed(pat_type) = main_arg {
            Some(pat_type)
        } else {
            None
        }
        .unwrap();

        let arg_name_ident = if let Pat::Ident(pat_ident) = &*main_arg_pat.pat {
            Some(pat_ident)
        } else {
            None
        }
        .unwrap();

        inner_main_args = quote! {
            #inner_main_args opts.#arg_name_ident,
        };
    }
    inner_main_args
}

fn collect_argument_macro_attributes(
    attributes: Vec<Attribute>
) -> (HashMap<String, TokenStream2>, Vec<Attribute>) {
    let mut argument_macro_attributes: HashMap<String, TokenStream2> = HashMap::new();
    let mut remaining_attributes: Vec<Attribute> = vec![];

    for attribute in attributes.clone().drain(..) {
        let path_ident = &attribute.path.segments.first().unwrap().ident;
        let macro_name = format!("{}", path_ident);

        if macro_name == "argument".to_string() {
            let attribute_token_stream = attribute.tokens.to_token_stream();

            for x in attribute_token_stream.into_iter() {
                let group = if let TokenTree2::Group(group) = x {
                    Some(group)
                } else {
                    None
                }
                .unwrap();

                let mut group_iter = group.stream().into_iter();
                let first_arg = group_iter.next().unwrap();
                let _first_arg_punc = group_iter.next();
                let rest: TokenStream2 = group_iter.collect();

                let first_arg_formatted = format!("{}", first_arg)
                    .strip_prefix("\"")
                    .unwrap()
                    .to_string()
                    .strip_suffix("\"")
                    .unwrap()
                    .to_string();

                argument_macro_attributes.insert(first_arg_formatted, rest);
            }
        } else {
            remaining_attributes.push(attribute);
        }
    }

    (argument_macro_attributes, remaining_attributes)
}

/// ## Note
/// All arguments available to the struct/enum level invocation of structopt are available to pass to `command` (https://docs.rs/structopt/0.3.23/structopt/#magical-methods).
#[proc_macro_attribute]
pub fn command(
    args_stream: TokenStream,
    input_stream: TokenStream,
) -> TokenStream {
    let args_stream = TokenStream2::from(args_stream);
    let input_itemfn = parse_macro_input!(input_stream as ItemFn);

    // Pick up all the additional macros that were applied to main.  These should
    //  all be "argument" proc macros.
    let attributes = input_itemfn.clone().attrs;
    let collect_result = collect_argument_macro_attributes(attributes);
    let argument_macro_attributes = collect_result.0;

    // TODO: Handle remaining attributes.  Imagine we'd want to figure out how to make
    //  sure things like the tokio async main macro still functions correctly and is
    //  applied to the correct function.
    let _remaining_attributes = collect_result.1;

    let structopt_block = build_structopt_block(
        args_stream.clone(),
        input_itemfn.clone(),
        argument_macro_attributes,
    );
    let inner_main_args = build_inner_main_args(input_itemfn.clone());

    // Pull out the parts of the input.
    let visibility = input_itemfn.vis;
    let body = input_itemfn.block;

    // Rename the original "main" function and define it as __inner_main so
    //  we can call it with the CLI arguments, and override main to have
    //  no arguments.
    let mut signature = input_itemfn.sig;
    signature.ident = format_ident!("__inner_main");

    // TODO: Add return type to main from original main.
    // TODO: Keep asyncness.
    let main_block = quote! {
        #structopt_block

        #visibility #signature #body

        #visibility fn main() {
            let opts = __CommandStruct::from_args();

            __inner_main(#inner_main_args);
        }
    };

    main_block.into()
}
