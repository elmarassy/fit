use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemStruct, parse_macro_input};

mod parse;
mod pdf;
mod translation;

extern crate proc_macro;

use intermediate_representation::binary_operation::BinaryOperation;
use intermediate_representation::expression::{ExpressionGraph, Node};

use pdf::PdfInput;

#[proc_macro]
pub fn define_pdf(input: TokenStream) -> TokenStream {
    let pdf_input = parse_macro_input!(input as PdfInput);

    let pdf_struct = &pdf_input.pdf_struct;
    let pdf_name = &pdf_struct.ident;

    let value_fn = &pdf_input.distribution;
    let norm_fn = &pdf_input.norm;
    let body = &value_fn.block.stmts[0];
    let expr = parse::build_graph(pdf_struct, value_fn).expect("Could not parse value function");
    let res = translation::translate(&expr, expr.len() - 1);
    println!("{}", res.to_string());
    let output = quote! {
        #pdf_struct
        #value_fn
        #norm_fn

        impl #pdf_name {
            #res
        }
    };

    TokenStream::from(output)
}
//
// #[proc_macro]
// pub fn define_pdf(input: TokenStream) -> TokenStream {
//     let args = parse_macro_input!(attr as pdf::PdfArgs);
//     let item = parse_macro_input!(item as ItemStruct);
//
//     if args.value.is_none() {
//         panic!("Must provide a value function for a pdf");
//     }
//     if let Some(value_fn) = args.value {
//         let expr = parse::build_graph(&item, value_fn).expect("Could not parse value function");
//         let res = translation::translate(&expr, expr.len() - 1);
//
//         let fields: Vec<_> = item.fields.iter().collect();
//     }
//
//     // let mut expr = ExpressionGraph::new();
//     // expr.insert(Node::new_variable("x".to_string(), false));
//     // expr.insert(Node::new_variable("y".to_string(), false));
//     // expr.nodes.push(Node::new_variable("z".to_string(), true));
//     //
//     // expr.nodes.push(Node::new_constant(3.0));
//     // expr.insert(Node::new_binary_operation(BinaryOperation::Div, 0, 1));
//     // expr.nodes.push(Node::new_builtin(Builtin::Sin, 2));
//     // expr.nodes
//     //     .push(Node::new_binary_operation(BinaryOperation::Mul, 4, 5));
//     //
//
//     let pdf_name = &item.ident;
//     //
//     // let value = args.value.map(|func_path| {
//     //     quote! {
//     //         pub fn value(&self) -> f64 {
//     //             #func_path(self.mu.value(), self.sigma.value(), self.x.data()[0])
//     //         }
//     //     }
//     // });
//
//     let norm = args.norm.map(|func_path| {
//         quote! {
//             pub fn norm(&self) -> f64 {
//                 #func_path()
//             }
//         }
//     });
//
//     let gradient = args.gradient.map(|func_path| {
//         quote! {
//             pub fn gradient(&self) -> f64 {
//                 #func_path()
//             }
//         }
//     });
//
//     TokenStream::from(quote! {
//         #item
//
//         impl #pdf_name {
//             pub fn evaluate(&self) -> f64 {
//                 0.0
//             }
//             // #value
//             #norm
//             #gradient
//         }
//     })
// }
