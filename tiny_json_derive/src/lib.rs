extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Data};
use syn::Attribute;
use syn::Meta;
use syn::Lit;

// Define a struct to represent the Rename attribute
struct RenameAttr(String);

impl RenameAttr {
    fn from_attributes(attrs: &[Attribute]) -> Option<Self> {
        attrs.iter().find_map(|attr| {
            if attr.path.is_ident("Rename") {
                match attr.parse_meta() {
                    Ok(Meta::NameValue(meta)) => {
                        if let Lit::Str(lit) = meta.lit {
                            Some(RenameAttr(lit.value()))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
    }
}

#[proc_macro_derive(Deserialize, attributes(Rename))]
pub fn deserialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            Fields::Unnamed(_) => panic!("Unnamed fields are not supported"),
            Fields::Unit => panic!("Unit structs are not supported"),
        },
        _ => panic!("Deserialize can only be implemented for structs"),
    };

    let deserialize_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let rename_attr = RenameAttr::from_attributes(&field.attrs);
        let field_key = rename_attr.map_or_else(
            || format!("{}", quote!(#field_name)),
            |RenameAttr(s)| s,
        );
        let ty = &field.ty;

        quote! {
        let #field_name = match value.get_value::<#ty>(&#field_key) {
            Ok(val) => val,
            Err(_) => return Err(serializer::DecodeError::ParseError),
        };
    }
    }).collect::<Vec<_>>();

    let field_initializers = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! { #field_name }
    }).collect::<Vec<_>>();

    let expanded = quote! {
        impl serializer::Deserialize for #name {
            fn deserialize(value: Option<&serializer::Value>) -> Result<Self, serializer::DecodeError> {
                let value = match value {
                    None => {return Err(serializer::DecodeError::ParseError)}
                    Some(v) => {v}
                };

                #(#deserialize_fields)*

                Ok(#name {
                    #(#field_initializers),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Serialize, attributes(Rename))]
pub fn serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Serialize currently only supports structs with named fields"),
        },
        _ => panic!("Serialize can only be implemented for structs"),
    };

    let field_transformations = fields.iter().map(|field| {
        let field_name = &field.ident;
        let rename_attr = RenameAttr::from_attributes(&field.attrs);
        let field_key = rename_attr.map_or_else(
            || format!("{}", quote!(#field_name)),
            |RenameAttr(s)| s,
        );

        quote! {
            object.insert(#field_key.to_owned(), self.#field_name.serialize());
        }
    });

    let expanded = quote! {
        impl serializer::Serialize for #name {
            fn serialize(&self) -> serializer::Value {
                let mut object = mapper::Object::new();
                #(#field_transformations)*
                mapper::Value::Object(object)
            }
        }
    };

    TokenStream::from(expanded)
}
