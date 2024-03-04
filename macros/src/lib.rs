use proc_macro2;
use quote::quote;
use syn;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(MakeOptional)]
pub fn make_optional(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let optional_name = format!("Optional{}", name);
    let optional_ident = Ident::new(&optional_name, name.span());

    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("MakeOptional can only be used with structs"),
    };

    let fields = match data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("MakeOptional can only be used with structs with named fields"),
    };

    let optional_fields = fields.named.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! { pub #name: Option<#ty> }
    });

    let field_names = fields.named.iter().map(|f| &f.ident);

    let set_key_value_cases = fields
        .named
        .iter()
        .map(|field| generate_set_field_case(field));

    let gen = quote! {
        #[derive(Default)]
        pub struct #optional_ident {
            #(#optional_fields,)*
        }

        impl #optional_ident {
            pub fn set_key_value(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
                match key {
                    #(#set_key_value_cases,)*
                    _ => Err("No such field"),
                }
            }
        }

        impl TryInto<#name> for #optional_ident {
            type Error = &'static str;

            fn try_into(self) -> Result<#name, Self::Error> {
                Ok(#name {
                    #(
                        #field_names: self.#field_names.ok_or(concat!("Missing field: ", stringify!(#field_names)))?,
                    )*
                })
            }
        }
    };

    gen.into()
}

fn generate_set_field_case(field: &Field) -> proc_macro2::TokenStream {
    let name = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    quote! {
        stringify!(#name) => {
            self.#name = Some(value.parse::<#ty>().map_err(|_| "Failed to parse field value")?);
            Ok(())
        }
    }
}
