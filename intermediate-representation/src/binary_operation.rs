use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    PowI,
    PowF,
}

impl BinaryOperation {
    pub fn generate_forward(
        &self,
        result: Ident,
        left_value: Ident,
        right_value: Ident,
    ) -> TokenStream {
        let num = format_ident!("Number");
        match &self {
            Self::Add => {
                quote! { let #result = #left_value + #right_value; }
            }
            Self::Sub => {
                quote! { let #result = #left_value - #right_value; }
            }
            Self::Mul => {
                quote! { let #result = #left_value * #right_value; }
            }
            Self::Div => {
                quote! { let #result = #left_value / #right_value; }
            }
            Self::PowI => {
                quote! { let #result = #left_value.powi(#right_value); }
            }
            Self::PowF => {
                quote! { let #result = #left_value.powf(#right_value as #num); }
            }
        }
    }
    pub fn generate_reverse(
        &self,
        propagate: Ident,
        left_value: Ident,
        right_value: Ident,
        left_adj: Ident,
        right_adj: Ident,
    ) -> TokenStream {
        let num = format_ident!("Number");
        match &self {
            Self::Add => {
                quote! {
                    #left_adj += #propagate;
                    #right_adj += #propagate;
                }
            }
            Self::Sub => {
                quote! {
                    #left_adj += #propagate;
                    #right_adj -= #propagate;
                }
            }
            Self::Mul => {
                quote! {
                    #left_adj += #right_value * #propagate;
                    #right_adj += #left_value * #propagate;
                }
            }
            Self::Div => {
                quote! {
                    #left_adj += #propagate / #right_value;
                    #right_adj -= #propagate * #left_value / (#right_value * #right_value);
                }
            }
            Self::PowI => {
                quote! { #left_adj += #propagate * #right_value as #num * #left_value.powi(#right_value - 1); }
            }
            Self::PowF => {
                quote! { #left_adj += #propagate * #right_value * #left_value.powf(#right_value - 1.0 as #num); }
            }
        }
    }
}
