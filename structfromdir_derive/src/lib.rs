use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(FromDir)]
pub fn from_dir(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields,
            _ => panic!("FromDir only supports named fields"),
        },
        _ => panic!("FromDir only supports structs"),
    };

    let field_idents = fields
        .named
        .iter()
        .map(|f| {
            f.ident
                .as_ref()
                .expect("Named field should have an identifier")
        })
        .collect::<Vec<_>>();

    let field_types = fields.named.iter().map(|f| &f.ty).collect::<Vec<_>>();

    let expanded = quote::quote! {
        #[automatically_derived]
        impl #ident {
            pub fn from_dir(dir: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
                use std::fs;
                use std::io::Read;
                use std::str::FromStr;
                use std::any::TypeId;

                let dir = dir.as_ref();
                let string_type_id = TypeId::of::<String>();

                #(
                    let mut #field_idents: #field_types = {
                        let mut data = fs::read_to_string(dir.join(stringify!(#field_idents)))?;
                        let data = if TypeId::of::<#field_types>() == string_type_id {
                            &data
                        } else {
                            data.trim()
                        };
                        #field_types::from_str(data).expect("TODO")
                    };
                )*

                Ok(Self {
                    #(#field_idents),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
