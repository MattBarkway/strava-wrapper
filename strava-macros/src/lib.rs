use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

fn derive_impl(
    input: TokenStream,
    trait_name: &syn::Ident,
    method_name: &syn::Ident,
    field_name: &str,
    return_type: &syn::Type,
    self_type: TokenStream2,
    as_mut: bool,
) -> TokenStream {
    let input = TokenStream2::from(input);
    let DeriveInput { ident, data, .. } = syn::parse2(input).expect("F");

    let field_ident = match &data {
        Data::Struct(data_struct) => {
            if let Fields::Named(fields_named) = &data_struct.fields {
                fields_named
                    .named
                    .iter()
                    .find(|f| f.ident.as_ref().map_or(false, |id| id == field_name))
                    .map(|f| &f.ident)
            } else {
                None
            }
        }
        _ => None,
    };

    let field_ident = &field_ident
        .expect("Expected a struct with the correct field name")
        .to_owned()
        .expect("Expected a struct with the correct field name");

    let expanded;
    if as_mut {
        expanded = quote! {
        impl #trait_name for #ident {
            fn #method_name(&mut self) -> &mut #return_type {
                &mut self.#field_ident
            }
        }};
    } else {
        expanded = quote! {
        impl #trait_name for #ident {
            fn #method_name(#self_type) -> #return_type {
                self.clone().#field_ident.clone()
            }
        }};
    }

    TokenStream::from(expanded)
}

// fn derive_default_impl(input: TokenStream, trait_name: &str) -> TokenStream {
//     let DeriveInput { ident, .. } = syn::parse(input).unwrap();
//
//     let expanded = quote! {
//         impl #trait_name for #ident {} {}
//     };
//
//     TokenStream::from(expanded)
// }

#[proc_macro_derive(Query)]
pub fn query_derive(input: TokenStream) -> TokenStream {
    let trait_name = syn::Ident::new("Query", proc_macro2::Span::call_site());
    let method_name = syn::Ident::new("query", proc_macro2::Span::call_site());
    let return_type = syn::parse_str::<syn::Type>("Vec<(String, String)>").unwrap();

    derive_impl(input, &trait_name, &method_name, "query", &return_type, quote! { mut self }, true)
}

#[proc_macro_derive(PathQuery)]
pub fn path_query_derive(input: TokenStream) -> TokenStream {
    let trait_name = syn::Ident::new("PathQuery", proc_macro2::Span::call_site());
    let method_name = syn::Ident::new("path_params", proc_macro2::Span::call_site());
    let return_type = syn::parse_str::<syn::Type>("Vec<(String, String)>").unwrap();


    derive_impl(
        input,
        &trait_name,
        &method_name,
        "path_params",
        &return_type,
        quote! { &self },
        true,
    )
}

#[proc_macro_derive(EndPoint)]
pub fn endpoint_derive(input: TokenStream) -> TokenStream {
    let trait_name = syn::Ident::new("EndPoint", proc_macro2::Span::call_site());
    let method_name = syn::Ident::new("path", proc_macro2::Span::call_site());
    let return_type = syn::parse_str::<syn::Type>("String").unwrap();

    derive_impl(input, &trait_name, &method_name, "path", &return_type, quote! { &self }, false)
}
