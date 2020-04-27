use proc_macro::TokenStream;
use quote::quote;

use crate::METHODS;

/// A macro for declaring methods callable for a Rusk Contract.
/// It needs to have an `opcode` specified.
pub fn method(input: syn::ItemFn) -> TokenStream {
  let name = &input.sig.ident;
  let ret = &input.sig.output;
  let body = &input.block;
  let vis = &input.vis;
  let inputs = &input.sig.inputs;

  let struct_args: Vec<syn::Ident> = inputs
    .iter()
    .filter_map(|i| match i {
      syn::FnArg::Typed(t) => match *(t.pat.clone()) {
        syn::Pat::Ident(ident) => {
          if ident.ident == "self" {
            None
          } else {
            Some(ident.ident)
          }
        }
        _ => panic!(
          "You have to use simple identifiers for delegated method parameters ({})",
          input.sig.ident
        ),
      },
      _ => None,
    })
    .collect();

  let struct_name = syn::Ident::new(&format!("{}_args", name), proc_macro2::Span::call_site());

  let struct_types: Vec<syn::Type> = inputs
    .iter()
    .filter_map(|i| match i {
      syn::FnArg::Typed(t) => Some(*t.ty.clone()),
      _ => None,
    })
    .collect();
  let result = if struct_types.is_empty() {
    quote! {
        #vis fn #name() #ret {
            #body
         }
    }
  } else {
    quote! {
        #[repr(C, packed)]
        pub struct #struct_name (
            #(#struct_types,)*
        );
        unsafe impl dataview::Pod for #struct_name {}

        #vis fn #name(#struct_name(#(#struct_args,)*): #struct_name) #ret {
            #body
         }
    }
  };

  let mut list = METHODS.lock().unwrap();
  list.push((name.to_string(), !struct_types.is_empty()));

  result.into()
}
