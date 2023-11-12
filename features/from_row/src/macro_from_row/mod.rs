mod fields;

use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;
use quote::quote;

pub fn execute_macro(input: DeriveInput) -> TokenStream2 {
  let struct_ident = input.ident.clone();

  let fields = match input.data {
    syn::Data::Struct(data) => 
      match fields::Fields::parse_struct(&struct_ident, data) {
        Ok(x) => x,
        Err(e) => panic!("{}: {}", struct_ident, e)
      },
    syn::Data::Enum(_) => panic!("FromRow [{}]: enums not supported", struct_ident),
    syn::Data::Union(_) => panic!("FromRow [{}]: unions not supported", struct_ident),
  };

  quote! {
    impl From<&postgres::Row> for #struct_ident {
      fn from(row: &postgres::Row) -> Self { 
        Self {
          #fields
        }
      }
    }
  }
}