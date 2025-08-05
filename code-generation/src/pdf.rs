extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, Item, ItemFn, ItemStruct, Path, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
};

pub struct PdfInput {
    pub pdf_struct: ItemStruct,
    pub distribution: ItemFn,
    pub norm: Option<ItemFn>,
}

impl Parse for PdfInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut pdf_struct = None;
        let mut distribution = None;
        let mut norm = None;
        while !input.is_empty() {
            let item: Item = input.parse()?;

            match item {
                Item::Struct(s) => {
                    if pdf_struct.is_some() {
                        return Err(syn::Error::new(
                            s.span(),
                            "PDF definition must have exactly one struct",
                        ));
                    }
                    pdf_struct = Some(s);
                }

                Item::Fn(f) => {
                    let fn_name = f.sig.ident.to_string();
                    match fn_name.as_str() {
                        "distribution" => {
                            if distribution.is_some() {
                                return Err(syn::Error::new(
                                    f.sig.ident.span(),
                                    "duplicate function definition for 'value'",
                                ));
                            }
                            distribution = Some(f);
                        }
                        "norm" => {
                            if norm.is_some() {
                                return Err(syn::Error::new(
                                    f.sig.ident.span(),
                                    "duplicate function definition for 'norm'",
                                ));
                            }
                            norm = Some(f);
                        }
                        _ => {
                            return Err(syn::Error::new(
                                f.sig.ident.span(),
                                "PDF definition can only contain 'distribution' and 'norm' functions",
                            ));
                        }
                    }
                }

                _ => {
                    return Err(syn::Error::new(
                        item.span(),
                        "Unexpected item. Only a single struct and functions named 'value', 'norm', or 'grad' are allowed.",
                    ));
                }
            }
            // =================================================================
        }

        let pdf_struct = pdf_struct.ok_or_else(|| {
            syn::Error::new(
                input.span(),
                "Missing struct definition inside the macro call.",
            )
        })?;
        let distribution = distribution.ok_or_else(|| {
            syn::Error::new(input.span(), "Missing required function `fn value(...)`.")
        })?;

        Ok(PdfInput {
            pdf_struct,
            distribution,
            norm,
        })
    }
    //     while !input.is_empty() {
    //         let lookahead = input.lookahead1();
    //         if lookahead.peek(Token![struct]) {
    //             let s: ItemStruct = input.parse()?;
    //             if struct_def.is_some() {
    //                 return Err(syn::Error::new(
    //                     s.span(),
    //                     "PDF definition must contain exactly one struct",
    //                 ));
    //             }
    //             struct_def = Some(s);
    //         } else if lookahead.peek(Token![fn]) {
    //             let f: ItemFn = input.parse()?;
    //             let fn_name = f.sig.ident.to_string();
    //
    //             match fn_name.as_str() {
    //                 "distribution" => {
    //                     if distribution.is_some() {
    //                         return Err(syn::Error::new(
    //                             f.sig.ident.span(),
    //                             "duplicate function definition for 'distribution'",
    //                         ));
    //                     }
    //                     distribution = Some(f);
    //                 }
    //                 "norm" => {
    //                     if norm.is_some() {
    //                         return Err(syn::Error::new(
    //                             f.sig.ident.span(),
    //                             "duplicate function definition for 'norm'",
    //                         ));
    //                     }
    //                     norm = Some(f);
    //                 }
    //                 _ => {
    //                     return Err(syn::Error::new(
    //                         f.sig.ident.span(),
    //                         "unrecognized function, PDF definition accepts 'distribution' or 'norm'",
    //                     ));
    //                 }
    //             }
    //         } else {
    //             return Err(lookahead.error());
    //         }
    //     }
    //
    //     let pdf_struct = struct_def.ok_or_else(|| {
    //         syn::Error::new(
    //             input.span(),
    //             "missing struct definition inside the macro call",
    //         )
    //     })?;
    //     let distribution = distribution.ok_or_else(|| {
    //         syn::Error::new(
    //             input.span(),
    //             "PDF is missing required function `fn distribution(...)`.",
    //         )
    //     })?;
    //
    //     Ok(PdfInput {
    //         pdf_struct,
    //         distribution,
    //         norm,
    //     })
    // }
}

//
// #[derive(Default)]
// pub struct PdfArgs {
//     pub value: Option<syn::Expr>,
//     pub norm: Option<syn::Expr>,
//     pub gradient: Option<syn::Expr>,
// }
//
// impl Parse for PdfArgs {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let mut args = PdfArgs::default();
//
//         while !input.is_empty() {
//             let ident: Ident = input.parse()?;
//             let key_str = ident.to_string();
//
//             input.parse::<Token![=]>()?;
//
//             match key_str.as_str() {
//                 "value" => {
//                     args.value = Some(input.parse()?);
//                 }
//                 "norm" => {
//                     args.norm = Some(input.parse()?);
//                 }
//                 "gradient" => {
//                     args.gradient = Some(input.parse()?);
//                 }
//                 _ => {
//                     return Err(syn::Error::new(
//                         ident.span(),
//                         format!("unknown argument `{}`", key_str),
//                     ));
//                 }
//             }
//
//             if input.peek(Token![,]) {
//                 input.parse::<Token![,]>()?;
//             } else if !input.is_empty() {
//                 return Err(input.error("expected comma between arguments"));
//             }
//         }
//         Ok(args)
//     }
// }
