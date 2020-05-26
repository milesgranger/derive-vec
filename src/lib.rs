#![allow(unused_imports, dead_code, unused_variables)]

use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(VecLike, attributes(vec))]
pub fn vec_like(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_ident = &ast.ident;

    let data = match &ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("#[derive(DataFrame)] only supported for structs"),
    };
    let field = data.fields
        .iter()
        .filter(|field| {
        field
            .attrs
            .iter()
            .filter(|attr| {
                match attr.parse_meta() {
                    Ok(meta) => {
                        if let syn::Meta::Path(path) = meta {
                            path.segments
                                .iter()
                                .any(|segment| &segment.ident.to_string() == "vec")
                        } else {
                            false
                        }
                    },
                    Err(_) => false
                }
            })
            .nth(0)
            .is_some()
        })
        .nth(0)
        .unwrap_or_else(|| panic!("Expected one attribute to have #[vec]"));

    dbg!(&field);

    // generate methods
    let fn_new = vec::new();
    let pub_fn_len = vec::len();
    let pub_fn_push = vec::push(&struct_ident);
    let pub_fn_remove = vec::remove(&struct_ident);
    let pub_fn_is_empty = vec::is_empty();

    (quote! {}).into()
}

mod vec {

    #![allow(unused_imports, dead_code)]

    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use syn::{FieldsNamed, Ident};

    /// Generate `DataFrame::push(row)` method
    pub fn new() -> TokenStream {
        quote! {
            fn new() -> Self {
                Self::default()
            }
        }
    }

    pub fn is_empty() -> TokenStream {
        quote! {
            pub fn is_empty(&self) -> bool {
                self.values.len() == 0
            }
        }
    }

    /// Generate `DataFrame::push(row)` method
    pub fn push(row_ident: &Ident) -> TokenStream {
        quote! {
            pub fn push(&mut self, row: #row_ident) {
                self.values.push(row)
            }
        }
    }

    pub fn len() -> TokenStream {
        quote! {
            pub fn len(&self) -> usize {
                self.values.len()
            }
        }
    }

    pub fn remove(row_ident: &Ident) -> TokenStream {
        quote! {
            pub fn remove(&mut self, idx: usize) -> #row_ident {
                self.values.remove(idx)
            }
        }
    }
}
