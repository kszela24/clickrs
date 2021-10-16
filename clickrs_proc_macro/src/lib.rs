extern crate proc_macro;

use darling::{FromMeta, ToTokens};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{parse_macro_input, AttributeArgs, FnArg, Ident, ItemFn, Pat, PatIdent, PatType};

#[proc_macro_attribute]
pub fn argument(
    args_stream: TokenStream,
    input_stream: TokenStream,
) -> TokenStream {
    let mut input_stream_string = input_stream.to_string();
    let input = parse_macro_input!(input_stream as ItemFn);

    println!("{}", args_stream.to_string());
    println!("{:?}", parse_macro_input!(args_stream as AttributeArgs));
    println!("{:?}", input_stream_string);
    println!("{:?}", input.attrs);

    if input.attrs.is_empty() {
        println!("{:?}", input.block.stmts);
        println!("{:?}", input.sig);
        input_stream_string = "fn main() { println!(\"hi\") }".to_string();
    }

    input_stream_string.parse().expect("Generated invalid tokens")
}

#[derive(Default, FromMeta)]
#[darling(default)]
struct ComandMacroArgs {
    name: String,
    bin_name: Option<String>,
    version: Option<String>,
    long_version: Option<String>,
    version_short: Option<String>,
    version_message: Option<String>,
    author: Option<String>,
    about: Option<String>,
    long_about: Option<String>,
    before_help: Option<String>,
    help: Option<String>,
    after_help: Option<String>,
    help_short: Option<String>,
    help_message: Option<String>,
    usage: Option<String>,
    template: Option<String>,
    set_term_width: Option<usize>,
    max_term_width: Option<usize>,
}

fn build_command_block(args: ComandMacroArgs) -> TokenStream2 {
    let command_name = args.name;

    let string_args_vec = vec![
        ("version", args.version),
        ("author", args.author),
        ("about", args.about),
        ("bin_name", args.bin_name),
        ("long_version", args.long_version),
        ("version_short", args.version_short),
        ("version_message", args.version_message),
        ("long_about", args.long_about),
        ("before_help", args.before_help),
        ("help", args.help),
        ("after_help", args.after_help),
        ("help_short", args.help_short),
        ("help_message", args.help_message),
        ("usage", args.usage),
        ("template", args.template),
    ];

    let usize_args_vec = vec![
        ("set_term_width", args.set_term_width),
        ("max_term_width", args.max_term_width),
    ];

    let mut command_block = quote! {
        use clap::{App, Arg};
        let matches = App::new(#command_name)
    };

    for (var_name, arg) in string_args_vec {
        let varname = format_ident!("{}", var_name);
        let command_builder_block = if let Some(val) = arg {
            quote! { .#varname(#val) }
        } else {
            quote! {}
        };

        command_block = quote! {
            #command_block
            #command_builder_block
        }
    }

    for (var_name, arg) in usize_args_vec {
        let varname = format_ident!("{}", var_name);
        let command_builder_block = if let Some(val) = arg {
            quote! { .#varname(#val) }
        } else {
            quote! {}
        };

        command_block = quote! {
            #command_block
            #command_builder_block
        };
    }

    command_block
}

#[proc_macro_attribute]
pub fn command(
    args_stream: TokenStream,
    input_stream: TokenStream,
) -> TokenStream {
    let attr_args = parse_macro_input!(args_stream as AttributeArgs);
    let args = match ComandMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    // pull out the parts of the input
    let input = parse_macro_input!(input_stream as ItemFn);
    let inputs = input.clone().sig.inputs;
    let _attributes = input.attrs;
    let visibility = input.vis;
    let body = input.block;
    let mut signature = input.sig;
    println!("{}", &signature.to_token_stream());
    signature.ident = format_ident!("__inner_main");

    let mut command_block = build_command_block(args);
    let mut matches_block = quote! {};
    let mut inner_main_args = quote! {};

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
        println!("{}", formatted_arg_name);
        command_block = quote! {
            #command_block
            .arg(Arg::with_name(#formatted_arg_name))
        };

        matches_block = quote! {
            #matches_block
            let #arg_name_ident = String::from(matches.value_of(#formatted_arg_name).unwrap());
        };

        inner_main_args = quote! {
            #inner_main_args
            #arg_name_ident,
        }
    }

    let main_block = quote! {
        #visibility #signature #body

        #visibility fn main() {
            #command_block.get_matches();
            #matches_block

            __inner_main(#inner_main_args);
        }
    };

    println!("{}", main_block);
    main_block.into()
}
