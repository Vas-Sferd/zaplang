mod terms;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Lit, Meta, parse_macro_input};

#[proc_macro_derive(ContextlessRule, attributes(choice_literal))]
pub fn derive_contextless_rule(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Обработка атрибута choice_literal
    let literals: Vec<String> = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("choice_literal"))
        .flat_map(|attr| match &attr.meta {
            Meta::List(list) => list.tokens.clone().into_iter(),
            _ => panic!("choice_literal attribute must be a list"),
        })
        .filter_map(|token| {
            if let proc_macro2::TokenTree::Literal(lit) = token {
                Some(lit.to_string().replace('\"', ""))
            } else {
                None
            }
        })
        .collect();

    if literals.is_empty() {
        panic!("At least one literal must be provided in choice_literal");
    }

    let match_arms = literals.iter().map(|lit| {
        quote! {
            if s.starts_with(#lit) {
                return Ok((&s[#lit.len()..], #lit));
            }
        }
    });

    let error_msg = format!("Expected one of: {}", literals.join(", "));

    let expanded = quote! {
        impl ContextlessRule for #name {
            fn extract(s: &str) -> Result<(&str, &str), String> {
                #(#match_arms)*
                Err(#error_msg.to_string())
            }
        }
    };

    TokenStream::from(expanded)
}
