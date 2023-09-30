use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{DataEnum, LitStr};

#[proc_macro_derive(SelectEnum, attributes(prompt))]
pub fn select_enum_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let (variants_names, variants) = match &input.data {
        syn::Data::Enum(data) => (as_str_match_arms(data), variants(data)),
        _ => panic!("SelectEnum can only be derived for enums"),
    };

    let indices = 0..variants.len();
    let indices2 = 0..variants.len();

    // Build the output tokens
    let name = &input.ident;
    // let variants = get_enum_variants(&input);
    let tokens = quote! {
        impl SelectEnum for #name {
            const VARIANTS: &'static [Self] = &[
                #(Self::#variants),*
            ];
            fn prompt(&self) -> &'static str {
                match self {
                    #(#variants_names),*
                }
            }
            fn to_index(&self) -> usize {
                match self {
                    #(Self::#variants => #indices),*
                }
            }
            fn from_index(n: usize) -> Option<Self> {
                match n {
                    #(#indices2 => Some(Self::#variants)),*,
                    _ => None
                }
            }
        }
    };

    // Return the output tokens
    tokens.into()
}

fn as_str_match_arms(data: &DataEnum) -> Vec<proc_macro2::TokenStream> {
    data.variants
        .iter()
        .map(|v| {
            let name = &v.ident;
            let attr = v.attrs.iter().find(|a| a.path().is_ident("prompt"));
            let prompt = attr.map(|a| a.parse_args::<LitStr>().expect("Expected string literal"));
            let prompt = prompt
                .map(|p| format!("{}", p.token()))
                .unwrap_or_else(|| format!("{}", name));
            quote! {
                Self::#name => #prompt
            }
        })
        .collect()
}

fn variants(data: &DataEnum) -> Vec<proc_macro2::Ident> {
    data.variants.iter().map(|v| v.ident.clone()).collect()
}
