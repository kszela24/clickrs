extern crate proc_macro;

use darling::{FromMeta, ToTokens};
use proc_macro::TokenStream;
use proc_macro::*;
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_str, AttributeArgs, Block, FnArg, Ident, ItemFn, Pat, PathArguments, ReturnType,
    Type,
};

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

#[derive(FromMeta)]
struct ComandMacroArgs {
    #[darling(default)]
    name: String,

    #[darling(default)]
    bin_name: Option<String>,

    #[darling(default)]
    version: Option<String>,

    #[darling(default)]
    long_version: Option<String>,

    #[darling(default)]
    version_short: Option<String>,

    #[darling(default)]
    version_message: Option<String>,

    #[darling(default)]
    author: Option<String>,

    #[darling(default)]
    about: Option<String>,

    #[darling(default)]
    long_about: Option<String>,

    #[darling(default)]
    before_help: Option<String>,

    #[darling(default)]
    help: Option<String>,

    #[darling(default)]
    after_help: Option<String>,

    #[darling(default)]
    help_short: Option<String>,

    #[darling(default)]
    help_message: Option<String>,

    #[darling(default)]
    usage: Option<String>,

    #[darling(default)]
    template: Option<String>,

    #[darling(default)]
    set_term_width: Option<usize>,

    #[darling(default)]
    max_term_width: Option<usize>,
}

fn get_command_block(args: ComandMacroArgs) -> TokenStream2 {
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
        }
    }

    command_block
}

#[proc_macro_attribute]
pub fn command(
    args_stream: TokenStream,
    input_stream: TokenStream,
) -> TokenStream {
    let cloned_input_stream = input_stream.clone();
    let attr_args = parse_macro_input!(args_stream as AttributeArgs);
    let args = match ComandMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let input = parse_macro_input!(input_stream as ItemFn);

    // pull out the parts of the input
    let _attributes = input.attrs;
    let visibility = input.vis;
    let signature = input.sig;
    let body = input.block;

    // pull out the parts of the function signature
    let fn_ident = signature.ident.clone();
    let inputs = signature.inputs.clone();
    let output = signature.output.clone();
    let asyncness = signature.asyncness;

    // pull out the names and types of the function inputs
    let input_tys = inputs
        .iter()
        .map(|input| match input {
            FnArg::Receiver(_) => panic!("methods (functions taking 'self') are not supported"),
            FnArg::Typed(pat_type) => pat_type.ty.clone(),
        })
        .collect::<Vec<Box<Type>>>();

    let input_names = inputs
        .iter()
        .map(|input| match input {
            FnArg::Receiver(_) => panic!("methods (functions taking 'self') are not supported"),
            FnArg::Typed(pat_type) => pat_type.pat.clone(),
        })
        .collect::<Vec<Box<Pat>>>();

    // pull out the output type
    let output_ty = match &output {
        ReturnType::Default => quote! {()},
        ReturnType::Type(_, ty) => quote! {#ty},
    };

    let output_span = output_ty.span();
    let output_ts = TokenStream::from(output_ty.clone());
    let output_parts = output_ts
        .clone()
        .into_iter()
        .filter_map(|tt| match tt {
            proc_macro::TokenTree::Ident(ident) => Some(ident.to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let output_string = output_parts.join("::");
    let output_type_display = output_ts.to_string().replace(" ", "");

    let command_block = get_command_block(args);
    let main_block = quote! {
        #visibility fn main() {
            #command_block.get_matches();

            println!("blep");
        }
    };
    println!("{:?}", command_block.to_string());

    println!("_attributes: {:?}", _attributes);
    println!("visibility: {:?}", visibility);
    println!("signature: {:?}", signature);
    println!("body: {:?}", body);
    println!("fn_ident: {:?}", fn_ident);
    println!("inputs: {:?}", inputs);
    println!("output: {:?}", output);
    println!("asyncness: {:?}", asyncness);

    println!("{}", main_block);
    main_block.into()
}
