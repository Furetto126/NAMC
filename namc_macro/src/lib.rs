extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, parse_quote};

#[proc_macro_attribute]
pub fn derive_animation(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: syn::ItemStruct = syn::parse(item).unwrap();

    if let Fields::Unit = input.fields {
        input.fields = Fields::Named(syn::FieldsNamed {
            brace_token: Default::default(),
            named: syn::punctuated::Punctuated::new()
        });
    }

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
            return syn::Error::new_spanned(&input.ident, "Only named structs are supported by #[derive_animation].")
                .to_compile_error()
                .into();
        }
    }

    let struct_name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Implementing Animation Trait
    // ----------------------------
    let trait_impl = quote! {
        impl #impl_generics namc_core::animation::Animation for #struct_name #ty_generics #where_clause {
            fn duration(&self) -> f64 {
                self.duration
            }

            fn update(&mut self, t: f64, scene_objects: &mut namc_core::scene::ObjectMap) {
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
        impl #impl_generics #struct_name #ty_generics #where_clause {
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

#[proc_macro_attribute]
pub fn derive_scene_object(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: syn::ItemStruct = syn::parse(item).unwrap();

    if let Fields::Unit = input.fields {
        input.fields = Fields::Named(syn::FieldsNamed {
            brace_token: Default::default(),
            named: syn::punctuated::Punctuated::new()
        });
    }

    match &mut input.fields {
        Fields::Named(fields_names) => {
            fields_names.named.push(parse_quote! {
                pub position: nalgebra::Vector3<f64>
            });

            fields_names.named.push(parse_quote! {
                pub opacity: f64
            });
        }
        _ => {
            return syn::Error::new_spanned(&input.ident, "Only named structs are supported by #[derive_scene_object].")
                .to_compile_error()
                .into();
        }
    }

    let struct_name = &input.ident;

    // Implementing SceneObject Trait
    // ------------------------------
    let trait_impl = quote! {
        impl namc_core::scene::SceneObject for #struct_name {
            fn as_any(&self) -> &dyn std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

            fn position(&self) -> nalgebra::Vector3<f64> {
                self.position
            }
            fn opacity(&self) -> f64 {
                self.opacity
            }
            fn set_position(&mut self, pos: nalgebra::Vector3<f64>) {
                self.position = pos;
            }
            fn set_opacity(&mut self, op: f64) {
                self.opacity = op;
            }
        }
    };

    quote! {
        #input
        #trait_impl
    }.into()
}