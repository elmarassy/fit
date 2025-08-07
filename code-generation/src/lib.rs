use intermediate_representation::{builtin::Builtin, expression::Node};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input, spanned::Spanned};

mod parse;
mod pdf;
mod translation;

extern crate proc_macro;

use pdf::PdfInput;
#[proc_macro_attribute]
pub fn define_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_mod: syn::ItemMod = parse_macro_input!(item as syn::ItemMod);
    let model_name = input_mod.ident.clone();
    let content = match input_mod.content {
        Some((_, items)) => items,
        None => {
            return syn::Error::new_spanned(
                input_mod,
                "#[define_model] only works on inline modules, not mod declarations",
            )
            .to_compile_error()
            .into();
        }
    };

    let body_ts = quote! {
        #(#content)*
    };

    let pdf_input = match syn::parse2::<PdfInput>(body_ts) {
        Ok(parsed) => parsed,
        Err(err) => return err.to_compile_error().into(),
    };

    let pdf_struct = &pdf_input.pdf_struct;
    let value_fn = &pdf_input.distribution;
    let likelihood_fn = &pdf_input.likelihood;
    let norm_fn = &pdf_input.norm;

    let mut value = match parse::build_graph(pdf_struct, value_fn) {
        Ok(e) => e,
        Err(e) => {
            return syn::Error::new_spanned(value_fn, e.to_string())
                .to_compile_error()
                .into();
        }
    };

    let res = translation::translate(
        &value,
        value.len() - 1,
        Ident::new("_value_and_gradient", value_fn.span()),
    );

    let likelihood = match likelihood_fn {
        Some(f) => match parse::build_graph(pdf_struct, f) {
            Ok(e) => translation::translate(
                &e,
                e.len() - 1,
                Ident::new("_likelihood", likelihood_fn.span()),
            ),
            Err(e) => {
                return syn::Error::new_spanned(value_fn, e.to_string())
                    .to_compile_error()
                    .into();
            }
        },
        None => {
            let l = Node::new_builtin(Builtin::Log, value.len() - 1);
            let index = value.insert(l);
            translation::translate(
                &value,
                index,
                Ident::new("_likelihood", likelihood_fn.span()),
            )
        }
    };

    let output = quote! {

        use intermediate_representation::{Float, FloatConsts};
        mod #model_name {
            use super::*;

            #pdf_struct
            #value_fn
            #norm_fn
            #likelihood_fn
            #likelihood
            #res
        }
    };

    output.into()
}
