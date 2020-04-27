use crate::method::method;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;

use crate::METHODS;

/// A macro for declaring methods callable for a Rusk Contract.
/// It needs to have an `opcode` specified.
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut result: Vec<proc_macro2::TokenStream> = vec![];
    let input = syn::parse_macro_input!(item as syn::ItemMod);
    let ident = &input.ident;

    let attrs = syn::parse_macro_input!(attr as syn::AttributeArgs);
    assert!(attrs.len() == 1, "only one attribute can be defined");

    let argument_name_and_value = match attrs.get(0) {
        Some(syn::NestedMeta::Meta(syn::Meta::NameValue(meta))) => meta,
        _ => panic!("expected argument `version = \"0.0.2\""),
    };
    let path = argument_name_and_value.path.get_ident().unwrap();
    assert!(
        *path == "version",
        format!(
            "Only version attribute can be set for contracts (found \"{}\")",
            path.to_string()
        )
    );

    let version = match &argument_name_and_value.lit {
        syn::Lit::Str(lit) => lit.value(),
        _ => panic!("expected argument value to be a u8"),
    };
    println!("CAKE CONTRACT VERSION: {}", version);
    println!("CONTRACT NAME: {}", ident.to_string());

    let (_, mod_items) = if let Some(content) = &input.content {
        content
    } else {
        panic!("Contracts cannot be empty.")
    };

    for mod_item in mod_items {
        match mod_item {
            syn::Item::Fn(item_fn) => {
                let vis = &item_fn.vis;
                if let syn::Visibility::Public(_) = vis {
                    result.push(method(item_fn.clone()).into());
                }
            }
            i => result.push(i.to_token_stream()),
        }
    }

    let values = METHODS.lock().unwrap();
    let keys = values.iter().enumerate().map(|(i, (m, _))| {
        println!("OPCODE {} FOR METHOD \"{}\"", i + 1, m);
        (i + 1) as u8
    });

    let values: Vec<syn::ExprCall> = values
        .iter()
        .map(|(k, a)| {
            let k = syn::Ident::new(k, proc_macro2::Span::call_site());
            if *a {
                syn::parse_quote! { #k(dusk_abi::argument()) }
            } else {
                syn::parse_quote! { #k() }
            }
        })
        .collect();

    (quote! {
        mod #ident {
            #(#result)*
        }

        #[no_mangle]
        pub fn call() -> i32 {
            let code: u8 = dusk_abi::opcode::<u8>();
            dusk_abi::ret::<i32>(match code {
                #( #keys => #ident::#values, ) *
                _ => 0,
            });
        }
    })
    .into()
}
