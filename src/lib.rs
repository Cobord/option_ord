extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, LitBool};

/// `#[transparent_option_ord(true)]` means you want the usual order on some and treat None as smaller than everything else
///
/// `#[transparent_option_ord(false)]` means you want the usual order on some and treat None as bigger than everything else
#[proc_macro_attribute]
pub fn transparent_option_ord(attr: TokenStream, item: TokenStream) -> TokenStream {
    let is_none_smallest = parse_macro_input!(attr as LitBool);
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident.clone();
    let generics = input.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let my_ord_impl = if is_none_smallest.value {
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                pub fn option_cmp(&self, other: &Self) -> std::cmp::Ordering {
                    match (&self.0, &other.0) {
                        (None, None) => std::cmp::Ordering::Equal,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (Some(a), Some(b)) => a.cmp(b),
                    }
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                pub fn option_cmp(&self, other: &Self) -> std::cmp::Ordering {
                    match (&self.0, &other.0) {
                        (None, None) => std::cmp::Ordering::Equal,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (Some(a), Some(b)) => a.cmp(b),
                    }
                }
            }
        }
    };
    let ord_impl = quote! {
        #[allow(clippy::non_canonical_partial_ord_impl)]
        impl #impl_generics PartialOrd for #name #ty_generics #where_clause {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.option_cmp(other))
            }
        }
        #[allow(clippy::non_canonical_ord_impl)]
        impl #impl_generics Ord for #name #ty_generics #where_clause {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.option_cmp(other)
            }
        }
    };

    let expanded = quote! {
        #[repr(transparent)]
        #[derive(PartialEq,Eq)]
        #input
        #my_ord_impl
        #ord_impl
    };

    TokenStream::from(expanded)
}
