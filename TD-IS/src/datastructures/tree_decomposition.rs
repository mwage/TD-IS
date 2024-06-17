use std::collections::VecDeque;

use super::{Bag, InputTreeDecomposition, Vertices};
use bit_vec::BitVec;

/// A nice Tree decomposition
#[derive(Debug)]
pub struct TreeDecomposition {
    nodes: Vec<Node>
}

impl TreeDecomposition {
    pub fn new(td_path: &str, vertices: &Vertices) -> Vec<Self> {
        let input_td = InputTreeDecomposition::new(td_path, &vertices);
        let mut tree_decompositions: Vec<TreeDecomposition> = Vec::new();
        let mut bag_treated = BitVec::from_elem(input_td.len(), false);
        for (i, v) in input_td.edges().iter().enumerate() {
            if v.is_empty() {
                bag_treated.set(i, true);
            }
        }

        while !bag_treated.all() {
            // Create a new TD from the first leaf
            let root_idx = match input_td.edges().iter().enumerate().find(|(i, _)| !bag_treated[*i]) {
                Some((i, _)) => i,
                None => panic!("No leaf left => cycle => input not a TD!")
            };

            tree_decompositions.push(Self::from_root(&input_td, &mut bag_treated, root_idx));
        }
        
        tree_decompositions
    }

    fn from_root(input_td: &InputTreeDecomposition, bag_treated: &mut BitVec, root_idx: usize) -> Self {
        let mut nodes = Vec::new();
        nodes.push(Node::new(Bag::new_empty(), NodeType::Root, usize::MAX)); // Empty dummy root node, so the rest works recursively from this
        Self::create_nodes(input_td, bag_treated, &mut nodes, root_idx, 0, &input_td.edges()[root_idx][..]);

        TreeDecomposition {
            nodes
        }
    }

    fn create_nodes(input_td: &InputTreeDecomposition, bag_treated: &mut BitVec, new_nodes: &mut Vec<Node>, curr_bag: usize, last_node: usize, neighbors: &[usize]) {
        let num_neighbors = neighbors.len();

        if num_neighbors > 1 {  // Need a join node, split off first neighbor
            new_nodes.push(Node::new(input_td.get_bag(curr_bag).clone(), NodeType::Join, last_node));
            Self::create_nodes(input_td, bag_treated, new_nodes, curr_bag, new_nodes.len(), &[neighbors[0]]);
            Self::create_nodes(input_td, bag_treated, new_nodes, curr_bag, new_nodes.len(), &neighbors[1..]);
            return;
        }

        // Only one neighbor, use introduce, forget, etc nodes.

        bag_treated.set(curr_bag, true);
    }
}

#[derive(Debug)]
pub struct Node {
    bag: Bag,
    node_type: NodeType,
    next: usize
}

impl Node {
    pub fn new(bag: Bag, node_type: NodeType, next: usize) -> Self {
        Node {
            bag,
            node_type,
            next
        }
    }
}

#[derive(Debug)]
enum NodeType {
    Leaf,
    Introduce,
    Forget,
    Join,
    Root
} 