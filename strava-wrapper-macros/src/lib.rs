extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ID)]
pub fn derive_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl ID for #name {
            fn id(mut self, id: u64) -> Self {
                self.path_params.push(("id".to_string(), id.to_string()));
                self
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Endpoint)]
pub fn derive_endpoint(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl Endpoint for #name {
            fn endpoint(&self) -> String {
                format!("{}/{}", self.url, self.path)
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Query)]
pub fn derive_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl Query for #name {
            fn get_query_params(self) -> Vec<(String, String)> {
                self.query
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(PathQuery)]
pub fn derive_path_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl PathQuery for #name {
            fn get_path_params(&self) -> HashMap<String, String> {
                self.path_params.iter().cloned().collect()
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Page)]
pub fn derive_page(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl Page for #name {
            fn page(mut self, number: u32) -> Self {
                self.path_params
                    .push(("page".to_string(), number.to_string()));
                self
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(PerPage)]
pub fn derive_per_page(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl PerPage for #name {
            fn per_page(mut self, size: u32) -> Self {
                self.query.push(("per_page".to_string(), size.to_string()));
                self
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(PageSize)]
pub fn derive_page_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl PageSize for #name {
            fn page_size(mut self, size: u32) -> Self {
                self.query.push(("page_size".to_string(), size.to_string()));
                self
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(AfterCursor)]
pub fn derive_after_cursor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl AfterCursor for #name {
            fn after_cursor(mut self, cursor: String) -> Self {
                self.query
                    .push(("after_cursor".to_string(), cursor.to_string()));
                self
            }
        }
    };

    expanded.into()
}
