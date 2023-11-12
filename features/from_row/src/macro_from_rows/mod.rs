use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;
use quote::quote;

pub fn execute_macro(input: DeriveInput) -> TokenStream2 {
  let ident = input.ident;

  quote! {
    impl #ident {
      pub fn from_rows(rows: Vec<postgres::Row>) -> Vec<Self> {
        rows.iter().map(|x| x.into()).collect::<Vec<#ident>>() 
      }
    }
  }
}