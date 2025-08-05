use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

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
                "#[define_pdf] only works on inline modules, not mod declarations",
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
    let norm_fn = &pdf_input.norm;

    let expr = match parse::build_graph(pdf_struct, value_fn) {
        Ok(e) => e,
        Err(e) => {
            return syn::Error::new_spanned(value_fn, e.to_string())
                .to_compile_error()
                .into();
        }
    };

    let res = translation::translate(&expr, expr.len() - 1);

    let output = quote! {
        mod #model_name {
            use super::*;

            #pdf_struct
            #value_fn
            #norm_fn
            #res
        }
    };

    output.into()
}
