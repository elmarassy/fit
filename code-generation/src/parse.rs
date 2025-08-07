use intermediate_representation::{
    Float, FloatConsts,
    binary_operation::BinaryOperation,
    builtin::Builtin,
    expression::{ExpressionGraph, Node, NodeId},
};

use std::collections::HashMap;
use syn::{
    Error, Expr, ExprPath, Fields, FnArg, Ident, ItemFn, ItemStruct, Pat, PatIdent, Result, Stmt,
    Type, TypePath, spanned::Spanned,
};

fn build_node(
    graph: &mut ExpressionGraph,
    node_map: &HashMap<Ident, NodeId>,
    expr: &Expr,
) -> Result<Node> {
    // let kind = match expr {
    //     Expr::MethodCall(_) => "MethodCall",
    //     Expr::Call(_) => "Call",
    //     Expr::Binary(_) => "Binary",
    //     Expr::Path(_) => "Path",
    //     Expr::Lit(_) => "Lit",
    //     Expr::Paren(_) => "Paren",
    //     Expr::Return(_) => "Return",
    //     other => {
    //         let tokens = other.to_token_stream();
    //         return Err(syn::Error::new_spanned(
    //             other,
    //             format!("Unknown Expr: {}", tokens),
    //         ));
    //     }
    // };
    // return Err(syn::Error::new_spanned(
    //     expr,
    //     format!("Expr kind: {}", kind),
    // ));
    //

    match expr {
        Expr::Binary(expr_bin) => {
            let left_node = build_node(graph, node_map, &expr_bin.left)?;
            let left = graph.insert(left_node);
            let right_node = build_node(graph, node_map, &expr_bin.right)?;
            let right = graph.insert(right_node);

            // let right = { graph.insert({ build_node(graph, node_map, &expr_bin.right)? }) };
            // let left = graph
            //     .get_node_index(build_node(graph, node_map, &expr_bin.left)?)
            //     .unwrap();
            // let right = graph
            //     .get_node_index(build_node(graph, node_map, &expr_bin.right)?)
            //     .unwrap();
            let binop = match &expr_bin.op {
                syn::BinOp::Add(_) => BinaryOperation::Add,
                syn::BinOp::Sub(_) => BinaryOperation::Sub,
                syn::BinOp::Mul(_) => BinaryOperation::Mul,
                syn::BinOp::Div(_) => BinaryOperation::Div,
                _ => {
                    return Err(syn::Error::new_spanned(
                        &expr_bin.op,
                        "Unsupported operation",
                    ));
                }
            };
            return Ok(Node::new_binary_operation(binop, left, right));
        }
        Expr::Path(ExprPath { path, .. }) => {
            let segments: Vec<_> = path.segments.iter().collect();
            if segments.len() == 1 {
                Ok(graph.get_node(*node_map.get(&path.segments[0].ident).unwrap()))
            } else if segments.len() == 2 && segments[0].ident == "Float" {
                match segments[1].ident.to_string().as_str() {
                    "PI" => return Ok(Node::new_float(Float::PI)),
                    "E" => return Ok(Node::new_float(Float::E)),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            expr,
                            format!("unsupported constant: {}", segments[1].ident.to_string()),
                        ));
                    }
                }
            } else {
                Err(syn::Error::new_spanned(
                    expr,
                    format!("unsupported constant"),
                ))
            }
        }
        Expr::Lit(syn::ExprLit { lit, .. }) => match lit {
            syn::Lit::Float(f) => Ok(Node::new_float(f.base10_parse::<Float>()?)),
            syn::Lit::Int(i) => Ok(Node::new_integer(i.base10_parse::<i32>()?)),
            _ => Err(syn::Error::new_spanned(lit, "Unsupported literal")),
        },
        Expr::Paren(inner) => build_node(graph, node_map, &inner.expr),
        Expr::Unary(expr_unary) => {
            if let syn::UnOp::Neg(_) = expr_unary.op {
                let inner_node = build_node(graph, node_map, &expr_unary.expr)?;
                if let Node::Constant(value) = inner_node {
                    let negative = Node::Constant(value.negate());
                    graph.insert(negative.clone());
                    return Ok(negative);
                }
                let index = graph.insert(inner_node);
                let zero = graph.insert(Node::Constant(
                    intermediate_representation::constant::Constant::Float(0.0),
                ));
                return Ok(Node::BinaryOperation(BinaryOperation::Sub, zero, index));
            }
            Err(syn::Error::new_spanned(
                expr_unary,
                "Unsupported unary operator",
            ))
        }
        Expr::Cast(expr_cast) => {
            let inner_node = build_node(graph, node_map, &expr_cast.expr)?;

            if let syn::Type::Path(type_path) = &*expr_cast.ty {
                let ident = &type_path.path.segments.last().unwrap().ident;
                let allowed = ["Float"];
                if !allowed.contains(&ident.to_string().as_str()) {
                    return Err(syn::Error::new_spanned(ident, "unsupported cast type"));
                }
            }

            Ok(inner_node)
        }
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            if method_call.args.is_empty() {
                if let Some(builtin) = Builtin::rust_mappings(&method_name) {
                    let receiver_node = build_node(graph, node_map, &method_call.receiver)?;
                    let receiver = graph.insert(receiver_node);
                    return Ok(Node::new_builtin(builtin, receiver));
                }
            }

            if method_call.args.len() == 1 {
                let binop = match method_name.as_str() {
                    "powi" => Some(BinaryOperation::PowI),
                    "powf" => Some(BinaryOperation::PowF),
                    _ => None,
                };

                if let Some(binop) = binop {
                    let left_node = build_node(graph, node_map, &method_call.receiver)?;
                    let right_node =
                        build_node(graph, node_map, method_call.args.first().unwrap())?;
                    let left = graph.insert(left_node);
                    let right = graph.insert(right_node);
                    return Ok(Node::new_binary_operation(binop, left, right));
                }
            }

            Err(syn::Error::new_spanned(
                method_call,
                format!("Unsupported method call: {}", method_name),
            ))
        }
        Expr::Call(call) => {
            if let Expr::Path(path) = &*call.func {
                let builtin = Builtin::rust_mappings(&path.path.segments[0].ident.to_string());
                if let Some(builtin) = builtin {
                    let node = build_node(graph, node_map, &call.args[0])?;
                    let arg = graph.insert(node);
                    // let arg = graph
                    //     .get_node_index(build_node(graph, node_map, &call.args[0])?)
                    //     .unwrap();
                    return Ok(Node::new_builtin(builtin, arg));
                }
            };
            Err(syn::Error::new_spanned(call, "Unsupported function call"))
        }

        Expr::Return(ret) => {
            let Some(ret_expr) = &ret.expr else {
                return Err(syn::Error::new_spanned(
                    ret,
                    "`return` without value is unsupported",
                ));
            };
            Ok(build_node(graph, node_map, ret_expr)?)
        }
        _ => Err(syn::Error::new_spanned(expr, "Unsupported expression")),
    }
}

pub fn build_graph(pdf_struct: &ItemStruct, value_function: &ItemFn) -> Result<ExpressionGraph> {
    let types = verify_types(pdf_struct, value_function).unwrap();

    let mut expression_graph = ExpressionGraph::new();

    let mut node_map: HashMap<Ident, NodeId> = HashMap::new();
    for arg in types.iter() {
        if let Type::Path(TypePath { path, .. }) = arg.1 {
            let type_ident = path.segments.last().unwrap().ident.clone();

            let variable = Node::new_variable(
                arg.0.to_string(),
                match type_ident.to_string().as_str() {
                    "Parameter" => false,
                    "Data" => true,
                    _ => {
                        return Err(Error::new(
                            pdf_struct.span(),
                            "PDF field types must be Parameter or Data",
                        ));
                    }
                },
            );
            node_map.insert(arg.0.clone(), expression_graph.insert(variable));
        }
    }

    for statement in &value_function.block.stmts {
        match statement {
            Stmt::Local(local) => {
                if let Pat::Ident(pattern_ident) = &local.pat {
                    if let Some(init) = &local.init {
                        let expr = build_node(&mut expression_graph, &node_map, &init.expr)?;
                        let id = { expression_graph.insert(expr) };
                        node_map.insert(pattern_ident.ident.clone(), id);
                    }
                }
            }
            Stmt::Expr(expr, ..) => {
                let expr = build_node(&mut expression_graph, &node_map, expr)?;
                expression_graph.insert(expr);
            }
            _ => {
                return Err(Error::new_spanned(statement, "Unsupported statement"));
            }
        }
    }

    println!("{:?}", expression_graph);
    Ok(expression_graph)
}

pub fn verify_types(
    pdf_struct: &ItemStruct,
    value_function: &ItemFn,
) -> Result<HashMap<Ident, Type>> {
    let fields = match &pdf_struct.fields {
        Fields::Named(named) => &named.named,
        _ => {
            return Err(Error::new(
                pdf_struct.span(),
                "PDF struct fields must be named",
            ));
        }
    };

    let mut types = HashMap::new();
    for field in fields {
        if let Some(ident) = &field.ident {
            if let Type::Path(TypePath { path, .. }) = &field.ty {
                if let Some(last) = path.segments.last() {
                    let type_ident = &last.ident;
                    if type_ident != "Parameter" && type_ident != "Data" {
                        return Err(Error::new(
                            type_ident.span(),
                            format!(
                                "Field `{}` must be of type `Parameter` or `Data`, found `{}`",
                                ident, type_ident
                            ),
                        ));
                    }
                } else {
                    return Err(Error::new(field.ty.span(), "Unsupported type path"));
                }
            } else {
                return Err(Error::new(
                    field.ty.span(),
                    "PDF struct attribute types must be Data or Parameter",
                ));
            }

            types.insert(ident.clone(), field.ty.clone());
        } else {
            return Err(Error::new(field.span(), "PDF struct fields must be named"));
        }
    }

    let mut value_args = Vec::new();
    for input in &value_function.sig.inputs {
        match input {
            FnArg::Typed(pattern_type) => match &*pattern_type.pat {
                Pat::Ident(PatIdent { ident, .. }) => {
                    value_args.push(ident.clone());
                }
                other => {
                    return Err(Error::new(other.span(), "Invalid argument"));
                }
            },
            FnArg::Receiver(_) => {
                return Err(Error::new(
                    input.span(),
                    "PDF value function cannot have method receiver",
                ));
            }
        }
    }

    let struct_field_names: Vec<_> = types.keys().cloned().collect();
    if value_args.len() != struct_field_names.len() {
        return Err(Error::new(
            value_function.sig.ident.span(),
            format!(
                "Mismatched count: struct {} has {} fields, but function {} has {} arguments",
                pdf_struct.ident.to_string(),
                struct_field_names.len(),
                value_function.sig.ident.to_string(),
                value_args.len()
            ),
        ));
    }

    for arg in &value_args {
        if !types.contains_key(arg) {
            return Err(Error::new(
                arg.span(),
                format!("Function argument `{}` not found in struct fields", arg),
            ));
        }
    }
    Ok(types)
}
