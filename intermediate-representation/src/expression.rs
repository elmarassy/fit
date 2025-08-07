use std::collections::{HashMap, HashSet};

use crate::Float;
use crate::binary_operation::BinaryOperation;
use crate::builtin::Builtin;
use crate::constant::Constant;
use crate::variable::Variable;

pub type NodeId = usize;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Node {
    Constant(Constant),
    Variable(Variable),
    Builtin(Builtin, NodeId),
    BinaryOperation(BinaryOperation, NodeId, NodeId),
}

impl Node {
    pub fn new_float(constant: Float) -> Node {
        Node::Constant(Constant::new_float(constant))
    }
    pub fn new_integer(constant: i32) -> Node {
        Node::Constant(Constant::new_int(constant))
    }
    pub fn new_variable(name: String, fixed: bool) -> Node {
        Node::Variable(Variable { name, fixed })
    }
    pub fn new_builtin(builtin: Builtin, argument: NodeId) -> Node {
        Node::Builtin(builtin, argument)
    }
    pub fn new_binary_operation(binop: BinaryOperation, left: NodeId, right: NodeId) -> Node {
        Node::BinaryOperation(binop, left, right)
    }
}

#[derive(Debug)]
pub struct ExpressionGraph {
    nodes: Vec<Node>,
    node_map: HashMap<Node, NodeId>,
}

impl ExpressionGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, node: Node) -> NodeId {
        if self.node_map.contains_key(&node) {
            return self.node_map.get(&node).unwrap().clone();
        }
        let index = self.nodes.len();
        self.node_map.insert(node.clone(), index);
        self.nodes.push(node);
        return index;
    }

    pub fn get_node_index(&self, node: Node) -> Option<NodeId> {
        let id = self.node_map.get(&node);
        if let Some(id) = id {
            return Some(id.clone());
        }
        None
    }

    pub fn get_node(&self, id: NodeId) -> Node {
        self.nodes[id].clone()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    fn get_children(&self, id: NodeId) -> Vec<NodeId> {
        match &self.nodes[id] {
            Node::BinaryOperation(_, left_id, right_id) => vec![*left_id, *right_id],
            Node::Builtin(_, argument_id) => vec![*argument_id],
            Node::Constant(_) | Node::Variable(_) => vec![],
        }
    }

    pub fn topological_sort(&self, start: NodeId) -> Vec<NodeId> {
        let mut visited = HashSet::new();
        let mut sorted = Vec::new();

        fn dfs(
            graph: &ExpressionGraph,
            node_id: NodeId,
            visited: &mut HashSet<NodeId>,
            sorted: &mut Vec<NodeId>,
        ) {
            if visited.contains(&node_id) {
                return;
            }
            visited.insert(node_id);

            for &child in &graph.get_children(node_id) {
                dfs(graph, child, visited, sorted);
            }
            sorted.push(node_id);
        }

        dfs(self, start, &mut visited, &mut sorted);
        sorted
    }
}
