use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use intermediate_representation::{
    constant::Constant,
    expression::{ExpressionGraph, Node, NodeId},
};

pub fn translate(graph: &ExpressionGraph, output_id: NodeId, signature: Ident) -> TokenStream {
    let mut parameter_map: HashMap<usize, usize> = HashMap::new();
    let mut data_map: HashMap<usize, usize> = HashMap::new();
    let sorted_nodes = graph.topological_sort(output_id);

    let val_name = |id: NodeId| format_ident!("v{}", id);
    let adj_name = |id: NodeId| format_ident!("a{}", id);

    let forward_pass_code: Vec<TokenStream> = sorted_nodes
        .iter()
        .map(|&id| {
            let node = graph.get_node(id);
            let result_name = val_name(id);
            match node {
                Node::Constant(number) => match number {
                    Constant::Float(value) => quote! { let #result_name = #value; },
                    Constant::Integer(value) => quote! { let #result_name = #value; },
                },
                Node::Variable(variable) => {
                    // let var_name = format_ident!("{}", variable.name);
                    // input_variable_ids.push(id);
                    if variable.fixed {
                        let data_index = data_map.len();
                        data_map.insert(id, data_index);
                        quote! { let #result_name = data[#data_index]; }
                    } else {
                        let parameter_index = parameter_map.len();
                        parameter_map.insert(id, parameter_index);
                        quote! {let #result_name = parameters[#parameter_index]; }
                    }
                    // quote! { let #result_name = #var_name; }
                }
                Node::Builtin(builtin, argument_id) => {
                    builtin.generate_forward(result_name, val_name(argument_id))
                }
                Node::BinaryOperation(binop, left_id, right_id) => {
                    binop.generate_forward(result_name, val_name(left_id), val_name(right_id))
                }
            }
        })
        .collect();

    let mut reverse_pass_code: Vec<TokenStream> = sorted_nodes
        .iter()
        .map(|&id| {
            let a_id = adj_name(id);
            if id == output_id {
                quote! { let mut #a_id = 1.0; }
            } else {
                quote! { let mut #a_id = 0.0; }
            }
        })
        .collect();

    for &id in sorted_nodes.iter().rev() {
        let node = graph.get_node(id);
        let propagate = adj_name(id);
        let reverse = match node {
            Node::Builtin(builtin, argument_id) => {
                builtin.generate_reverse(propagate, val_name(argument_id), adj_name(argument_id))
            }
            Node::BinaryOperation(binop, left_id, right_id) => binop.generate_reverse(
                propagate,
                val_name(left_id),
                val_name(right_id),
                adj_name(left_id),
                adj_name(right_id),
            ),
            Node::Constant(_) | Node::Variable(_) => quote! {},
        };
        reverse_pass_code.push(reverse);
    }

    let final_value_name = val_name(output_id);

    let data_cols = data_map.len();
    let data = quote! {
        data: [Number; #data_cols]
    };

    let parameter_cols = parameter_map.len();
    let parameters = quote! {
        parameters: [Number; #parameter_cols]
    };

    let function_signature = quote! {
        pub fn #signature(#parameters, #data) -> (Number, [Number; #parameter_cols])
    };

    let mut input_adj_names = Vec::new();
    input_adj_names.resize(parameter_cols, adj_name(0));

    parameter_map.iter().for_each(|(&key, &val)| {
        input_adj_names[val] = adj_name(key);
    });

    let b = quote! {
        #function_signature {
            #(#forward_pass_code)*
            #(#reverse_pass_code)*
            let final_value = #final_value_name;
            let gradient = [#(#input_adj_names),*];
            (final_value, gradient)
        }
    };
    println!("{}", b.to_string());
    b
}
