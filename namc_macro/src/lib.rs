extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, parse_quote};

#[proc_macro_attribute]
pub fn animation(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: syn::ItemStruct = syn::parse(item).unwrap();

    match &mut input.fields {
        Fields::Named(fields_names) => {
            fields_names.named.push(parse_quote! {
                pub duration: f64
            });

            fields_names.named.push(parse_quote! {
                pub interpolation_function: namc_core::math::InterpolationFunction
            });
        }
        _ => {
            return syn::Error::new_spanned(&input.ident, "Only named structs are supported by #[Animation].")
                .to_compile_error()
                .into();
        }
    }

    let struct_name = &input.ident;

    // Implementing Animation Trait
    // ----------------------------
    let trait_impl = quote! {
        impl namc_core::animation::Animation for #struct_name {
            fn duration(&self) -> f64 {
                self.duration
            }

            fn update(&self, t: f64, scene_objects: &mut namc_core::scene::ObjectMap) {
                let anim_t = (self.interpolation_function)(t);
                self.animate(anim_t, scene_objects);
            }
        }
    };

    // Implementing "new()" function
    // -----------------------------
    let params_defs: Vec<_> = match &input.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|f| {
                let name = &f.ident;
                let ty   = &f.ty;
                quote! { #name: #ty }
            })
            .collect(),
        _ => vec![]
    };

    let params_init: Vec<_> = match &input.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|f| {
                let name = &f.ident;
                quote! { #name }
            })
            .collect(),
        _ => vec![]
    };

    let new_impl = quote! {
        impl #struct_name {
            pub fn new(#(#params_defs),*) -> Box<Self> {
                Box::new(Self {
                    #(#params_init),*
                })
            }
        }
    };

    quote! {
        #input
        #trait_impl
        #new_impl
    }.into()
}

