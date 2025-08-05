use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Builtin {
    Sin,
    Cos,
    Tan,
    Exp,
    Log,
}

impl Builtin {
    pub fn rust_mappings(name: &str) -> Option<Self> {
        match name {
            "sin" => Some(Self::Sin),
            "cos" => Some(Self::Cos),
            "tan" => Some(Self::Tan),
            "exp" => Some(Self::Exp),
            "ln" => Some(Self::Log),
            _ => None,
        }
    }

    pub fn generate_forward(&self, result: Ident, argument_value: Ident) -> TokenStream {
        match &self {
            Self::Sin => {
                quote! { let #result = #argument_value.sin(); }
            }
            Self::Cos => {
                quote! { let #result = #argument_value.cos(); }
            }
            Self::Tan => {
                quote! { let #result = #argument_value.tan(); }
            }
            Self::Exp => {
                quote! { let #result = #argument_value.exp(); }
            }
            Self::Log => {
                quote! { let #result = #argument_value.ln(); }
            }
        }
    }
    pub fn generate_reverse(
        &self,
        propagate: Ident,
        argument_value: Ident,
        argument_adj: Ident,
    ) -> TokenStream {
        match &self {
            Self::Sin => {
                quote! { #argument_adj += #propagate * #argument_value.cos(); }
            }
            Self::Cos => {
                quote! { #argument_adj -= #propagate * #argument_value.sin(); }
            }
            Self::Tan => {
                quote! { #argument_adj += #propagate * #argument_value.cos().powi(-2); }
            }
            Self::Exp => {
                quote! { #argument_adj += #propagate * #argument_value.exp(); }
            }
            Self::Log => {
                quote! { #argument_adj += #propagate / #argument_value; }
            }
        }
    }
}
