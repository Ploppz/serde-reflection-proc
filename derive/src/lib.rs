extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Type};

#[proc_macro_derive(Reflection)]
pub fn reflection(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    impl_reflection(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn impl_reflection(input: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let cr = syn::Ident::new("serde_reflection", Span::call_site());
    let self_cr = syn::Ident::new("serde_reflection_proc", Span::call_site());
    let mut format_gen = quote! {};
    let ident = input.ident.clone();
    match input.data {
        syn::Data::Struct(struct_data) => {
            format_gen = quote_format_for_fields(&struct_data.fields, false);
        }
        syn::Data::Enum(enum_data) => {
            // TODO: #[serde(tag = ..)]
            let ident = enum_data.variants.iter().map(|variant| &variant.ident);
            // List of variant formats
            let variant_format = enum_data
                .variants
                .iter()
                .map(|variant| quote_format_for_fields(&variant.fields, true));
            let index = 0u32..;
            format_gen = quote! {{
                use std::collections::BTreeMap;
                let mut variants = BTreeMap::new();
                #(
                    variants.insert(#index, #cr::Named {
                        name: stringify!(#ident).to_string(),
                        value: #variant_format
                    });
                )*
                #cr::ContainerFormat::Enum(variants)
            }}
        }
        syn::Data::Union(_) => unimplemented!(),
    }

    Ok(quote! {
        impl #self_cr::Reflection for #ident {
            fn get_format() -> Result<#cr::Format, String> {
                Ok(#cr::Format::TypeName(stringify!(#ident).to_string()))

            }
            fn get_container_format() -> Result<#cr::ContainerFormat, String> {
                Ok(#format_gen)
            }
        }

    })
}

#[derive(PartialEq, Eq)]
enum FieldAttr {
    Flatten,
}
fn parse_field_attr(attr: &syn::Attribute) -> Vec<FieldAttr> {
    let mut attrs = Vec::new();
    match attr.parse_meta().ok() {
        Some(syn::Meta::List(list)) => {
            if list.path.is_ident("serde") {
                for nested in list.nested.iter() {
                    // #[serde(flatten)]
                    match nested {
                        syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                            if path.is_ident("flatten") {
                                attrs.push(FieldAttr::Flatten);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    attrs
}

/// Generate (quote) code for constructing format for `syn::Fields`
/// If `variant_format` is true, construct `VariantFormat` rather than `ContainerFormat`
fn quote_format_for_fields(fields: &syn::Fields, variant_format: bool) -> TokenStream {
    let cr = syn::Ident::new("serde_reflection", Span::call_site());
    let self_cr = syn::Ident::new("serde_reflection_proc", Span::call_site());
    let (construct_struct, construct_unit) = if variant_format {
        (
            quote! {#cr::VariantFormat::Struct},
            quote! {
                #cr::VariantFormat::Unit
            },
        )
    } else {
        (
            quote! {#cr::ContainerFormat::Struct},
            quote! {#cr::VariantFormat::Unit},
        )
    };
    match fields {
        syn::Fields::Named(ref fields) => {
            let ty = fields.named.iter().map(|field| &field.ty);
            let name = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap().to_string());
            let attrs = fields
                .named
                .iter()
                .map(|field| field.attrs.iter().map(|x| parse_field_attr(x)).flatten());
            let flatten = attrs.map(|mut attr| attr.any(|a| a == FieldAttr::Flatten));
            quote! {{
                let mut fields: Vec<serde_reflection::Named<serde_reflection::Format>> = Vec::new();
                #(
                    if #flatten {
                        let container_format = <#ty as #self_cr::Reflection>::get_container_format()?;
                        match container_format {
                            #construct_struct (inner_fields) => {
                                fields.extend(inner_fields);

                            }
                            _ => return Err("Can only flatten struct types".to_string()),
                        }
                    } else {
                        let field_format = <#ty as #self_cr::Reflection>::get_format()?;
                        fields.push(#cr::Named {
                            name: #name.to_string(),
                            value: field_format
                        })
                    }
                )*

                #construct_struct (fields)
            }}
        }
        syn::Fields::Unnamed(ref fields) => unimplemented!(),
        syn::Fields::Unit => quote! {
            #construct_unit
        },
    }
}
