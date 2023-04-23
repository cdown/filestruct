use proc_macro::TokenStream;

use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr, PathArguments, Type};

fn get_file_attribute(field: &syn::Field) -> Result<Option<String>, syn::parse::Error> {
    let mut filename = None;

    for attr in &field.attrs {
        if attr.path().is_ident("structfromdir") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("file") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    filename = Some(s.value());
                    Ok(())
                } else {
                    Err(meta.error("unsupported attribute"))
                }
            })?;
        }
    }

    Ok(filename)
}

#[proc_macro_derive(FromDir, attributes(structfromdir))]
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

    let field_parsers = fields
        .named
        .iter()
        .map(|f| {
            let field_ident = f
                .ident
                .as_ref()
                .expect("Named field should have an identifier");
            let field_ty = &f.ty;
            let file_name = get_file_attribute(f).expect("Invalid file attribute");
            let file_name = file_name.unwrap_or_else(|| field_ident.to_string());
            match field_ty {
                Type::Path(type_path)
                    if type_path.path.segments.last().unwrap().ident == "Option" =>
                {
                    let inner_ty = match &type_path.path.segments.last().unwrap().arguments {
                        PathArguments::AngleBracketed(inner_ty) => &inner_ty.args[0],
                        _ => panic!("Unsupported Option type"),
                    };
                    quote::quote! {
                        let path = dir.join(#file_name);
                        let #field_ident: #field_ty = {
                            if let Ok(raw_data) = fs::read_to_string(path) {
                                let data = if TypeId::of::<#inner_ty>() == string_type_id {
                                    &raw_data
                                } else {
                                    raw_data.trim()
                                };
                                #inner_ty::from_str(data).ok()
                            } else {
                                None
                            }
                        };
                    }
                }
                _ => {
                    quote::quote! {
                        let path = dir.join(#file_name);
                        let raw_data = fs::read_to_string(&path)?;
                        let data = if TypeId::of::<#field_ty>() == string_type_id {
                            &raw_data
                        } else {
                            raw_data.trim()
                        };
                        let #field_ident: #field_ty = #field_ty::from_str(data)
                            .map_err(|_| structfromdir::Error::Parse {
                                file: path,
                                input: raw_data,
                                ty: stringify!(#field_ty).to_string()
                            })?;
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let field_idents = fields
        .named
        .iter()
        .map(|f| {
            f.ident
                .as_ref()
                .expect("Named field should have an identifier")
        })
        .collect::<Vec<_>>();

    let expanded = quote::quote! {
        #[automatically_derived]
        impl #ident {
            pub fn from_dir(dir: impl AsRef<std::path::Path>) -> Result<Self, structfromdir::Error> {
                use std::fs;
                use std::str::FromStr;
                use std::any::TypeId;

                let dir = dir.as_ref();
                let string_type_id = TypeId::of::<String>();

                #(#field_parsers)*

                Ok(Self {
                    #(#field_idents),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
