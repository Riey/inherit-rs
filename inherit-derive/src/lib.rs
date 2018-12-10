#![recursion_limit = "128"]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::DeriveInput;
use proc_macro::Ident;

#[proc_macro_derive(Inherit)]
pub fn derive_inherit(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    if let syn::Data::Struct(target_struct) = ast.data {
        let base_field = target_struct.fields.iter().find(|f| {
            f.ident.as_ref().map(|name| {
                let name = name.to_string();
                if name == "base" || name == "inner" {
                    true
                } else {
                    false
                }
            }).unwrap_or(false)
        }).unwrap();

        let input_ty = &ast.ident;
        let base_field_name = base_field.ident.as_ref().unwrap();
        let base_ty = &base_field.ty;
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        let ret = quote! {
            impl#impl_generics AsRef<#base_ty> for #input_ty#ty_generics #where_clause {
                fn as_ref(&self) -> &#base_ty {
                    &self.#base_field_name
                }
            }

            impl#impl_generics AsMut<#base_ty> for #input_ty#ty_generics #where_clause {
                fn as_mut(&mut self) -> &mut #base_ty {
                    &mut self.#base_field_name
                }
            }

            impl#impl_generics ::std::ops::Deref for #input_ty#ty_generics #where_clause {
                type Target = #base_ty;

                fn deref(&self) -> &#base_ty {
                    &self.#base_field_name
                }
            }

            impl#impl_generics ::std::ops::DerefMut for #input_ty#ty_generics #where_clause {
                fn deref_mut(&mut self) -> &mut #base_ty {
                    &mut self.#base_field_name
                }
            }

            impl#impl_generics ::inherit::Inherit<#base_ty> for #input_ty#ty_generics #where_clause {

            }
        };

        ret.into()

    } else {
        panic!("target is not struct");
    }

}

