extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Data};


#[proc_macro_derive(Deserialize)]
pub fn deserialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = if let Data::Struct(data_struct) = &input.data {
        match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            Fields::Unnamed(_) => panic!("Unnamed fields are not supported"),
            Fields::Unit => panic!("Unit structs are not supported"),
        }
    } else {
        panic!("Deserialize can only be implemented for structs");
    };

    let deserialize_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        quote! {
            let #field_name = match value.get_value::<#ty>(stringify!(#field_name)) {
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

#[proc_macro_derive(Serialize)]
pub fn serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("serializer::Serialize currently only supports structs with named fields"),
        },
        _ => panic!("serializer::Serialize can only be implemented for structs"),
    };

    let field_transformations = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            object.insert(stringify!(#field_name).to_owned(), self.#field_name.serialize());
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