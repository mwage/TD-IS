use super::{Bag, InputTreeDecomposition, Graph};
use bit_vec::BitVec;

/// A nice Tree decomposition
#[derive(Debug)]
pub struct TreeDecomposition {
    nodes: Vec<Node>
}

impl TreeDecomposition {
    pub fn new(td_path: &str, graph: &Graph) -> Vec<Self> {
        let input_td = InputTreeDecomposition::new(td_path, &graph);
        let mut tree_decompositions: Vec<TreeDecomposition> = Vec::new();
        let mut bag_treated = BitVec::from_elem(input_td.len(), false);
        for (i, v) in input_td.edges().iter().enumerate() {
            if v.is_empty() {
                bag_treated.set(i, true);
            }
        }

        while !bag_treated.all() {
            // Create a new TD with the first leaf as a root
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
        let mut leaves = Vec::new();
        nodes.push(Node::new(Bag::new_empty(), NodeType::Root, usize::MAX)); // Empty dummy root node, so the rest works recursively from this
        Self::create_nodes(input_td, bag_treated, &mut nodes, &mut leaves, root_idx, 0, &input_td.edges()[root_idx]);

        TreeDecomposition {
            nodes
        }
    }

    fn create_nodes(input_td: &InputTreeDecomposition, bag_treated: &mut BitVec, new_nodes: &mut Vec<Node>, leaves: &mut Vec<usize>, curr_bag: usize, last_node: usize, neighbors: &[usize]) {
        let num_neighbors = neighbors.len();
        let bag = input_td.get_bag(curr_bag);

        if num_neighbors > 1 {  // Need a join node, split off first neighbor
            new_nodes.push(Node::new(bag.clone(), NodeType::Join, last_node));
            let last_idx = new_nodes.len() - 1;
            Self::create_nodes(input_td, bag_treated, new_nodes, leaves, curr_bag, last_idx, &[neighbors[0]]);
            Self::create_nodes(input_td, bag_treated, new_nodes, leaves, curr_bag, last_idx, &neighbors[1..]);
            return;
        }

        if num_neighbors == 0 { // No neighbor -> leaf node
            leaves.push(new_nodes.len());
            new_nodes.push(Node::new(bag.clone(), NodeType::Leaf, last_node));
            bag_treated.set(curr_bag, true);
            return;
        }

        // Exactly one neighbor, use introduce, forget, etc nodes.
        let neighbor_idx = neighbors[0];
        let neighbor = input_td.get_bag(neighbor_idx);
        let to_introduce = bag.vertices().iter().filter(|v| !neighbor.vertices().contains(*v)).map(|x| *x).collect::<Vec<usize>>();
        let to_forget = neighbor.vertices().iter().filter(|v| !bag.vertices().contains(*v)).map(|x| *x).collect::<Vec<usize>>();

        // Add introduce nodes
        let mut working_bag = bag.vertices().clone();
        let mut prev_node_idx = last_node;
        for vertex in to_introduce.into_iter() {
            new_nodes.push(Node::new(Bag::new(working_bag.clone()), NodeType::Introduce(vertex), prev_node_idx));
            working_bag.retain(|v| *v != vertex);   // Remove element from working bag
            prev_node_idx = new_nodes.len() - 1;
        }
        // Add forget nodes
        for vertex in to_forget.into_iter() {
            new_nodes.push(Node::new(Bag::new(working_bag.clone()), NodeType::Forget(vertex), prev_node_idx));
            working_bag.push(vertex);   // Remove element from working bag
            prev_node_idx = new_nodes.len() - 1;
        }

        bag_treated.set(curr_bag, true);

        let neighbors = input_td.edges()[neighbor_idx].iter().filter(|v| !bag_treated[**v]).map(|x| *x).collect::<Vec<usize>>();
        Self::create_nodes(input_td, bag_treated, new_nodes, leaves, neighbor_idx, prev_node_idx, &neighbors);
    }
}

#[derive(Debug)]
pub struct Node {
    bag: Bag,
    node_type: NodeType,
    next: usize,
    // TODO: Add Hashmap or vec for dyn programming
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
    Introduce(usize),
    Forget(usize),
    Join,
    Root
} 