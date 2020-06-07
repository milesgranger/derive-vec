use quote::quote;
use syn::{Data, DeriveInput, Ident};

#[proc_macro_derive(VecBehavior, attributes(vec))]
pub fn vec_like(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_ident = &ast.ident;

    let data = match &ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("#[derive(DataFrame)] only supported for structs"),
    };

    let field = decorated_field(&data.fields);
    let inner_vec_ident = vec_inner_type(&field.ty);
    let field_name = field
        .ident
        .as_ref()
        .unwrap_or_else(|| todo!("#[vec] required on named fields"));

    (quote! {
        impl VecTrait<#inner_vec_ident> for #struct_ident {
            fn append(&mut self, other: &mut Vec<#inner_vec_ident>) {
                self.#field_name.append(other)
            }
            fn as_slice(&self) -> &[#inner_vec_ident] {
                self.#field_name.as_slice()
            }
            fn capacity(&self) -> usize {
                self.#field_name.capacity()
            }
            fn clear(&mut self) {
                self.#field_name.clear()
            }
            fn dedup(&mut self) {
                self.#field_name.dedup()
            }
            fn dedup_by<F>(&mut self, same_bucket: F)
                where
                    F: FnMut(&mut #inner_vec_ident, &mut #inner_vec_ident) -> bool
            {
                self.#field_name.dedup_by(same_bucket)
            }
            fn dedup_by_key<F, K>(&mut self, key: F)
                where
                    F: FnMut(&mut #inner_vec_ident) -> K,
                    K: PartialEq<K>
            {
                self.#field_name.dedup_by_key(key)
            }
            fn len(&self) -> usize {
                self.#field_name.len()
            }
            fn is_empty(&self) -> bool {
                self.#field_name.is_empty()
            }
            fn push(&mut self, val: #inner_vec_ident) {
                self.#field_name.push(val)
            }
            fn with_capacity(capacity: usize) -> Self {
                Self {
                    #field_name: Vec::with_capacity(capacity),
                    ..Self::default()
                }
            }
        }

    })
    .into()
}

// Find the field which as been decorated with #[vec] to
// denote which attribute to treat as the Vec
fn decorated_field(fields: &syn::Fields) -> &syn::Field {
    fields
        .iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .filter(|attr| match attr.parse_meta() {
                    Ok(meta) => {
                        if let syn::Meta::Path(path) = meta {
                            path.segments
                                .iter()
                                .any(|segment| &segment.ident.to_string() == "vec")
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                })
                .nth(0)
                .is_some()
        })
        .nth(0)
        .unwrap_or_else(|| panic!("Expected one attribute to have #[vec]"))
}

// Get the inner ident from a Vec type. ie: `T` from `Vec<T>`
fn vec_inner_type(ty: &syn::Type) -> Ident {
    match ty {
        syn::Type::Path(path) => {
            let v = &path.path.segments.iter().nth(0).unwrap();
            if &v.ident.to_string() != "Vec" {
                panic!("#[vec] attribute must be a Vec<..> type")
            }
            match &v.arguments {
                syn::PathArguments::AngleBracketed(vec_args) => {
                    match &vec_args.args.iter().nth(0).unwrap() {
                        syn::GenericArgument::Type(ty) => {
                            match &ty {
                                syn::Type::Path(type_path) => {
                                    // This is the ie 'T' of a Vec<T>
                                    type_path.path.segments.iter().nth(0).unwrap().ident.clone()
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => {
                            unreachable!("Appears attribute's Vec<> doesn't contain an inner type?")
                        }
                    }
                }
                _ => unreachable!("Appears attribute's `Vec` doesn't contain angle brackets?"),
            }
        }
        _ => panic!("Only Vec<..> types supported for a #[vec] attribute"),
    }
}
