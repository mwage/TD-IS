use std::collections::VecDeque;

use bit_vec::BitVec;
use itertools::Itertools;

use super::*;


pub struct ISSolver {}

impl ISSolver {
    pub fn solve(td: &TreeDecomposition, graph: &Graph) -> usize {
        let mut finished_leaves = BitVec::from_elem(td.leaves().len(), false);
        
        while !finished_leaves.all() {
            let current_leaf = finished_leaves.iter().enumerate().find(|(_, b)| !*b).unwrap().0;
            let mut current_node = td.leaves()[current_leaf];
            // Traverse from leaf to next unprocessed join (or root)
            while td.get_node(current_node).can_traverse(&td) {
                td.get_node_mut(current_node).update_entries(graph, td);
                current_node = td.get_node(current_node).next();
            }
            finished_leaves.set(current_leaf, true);
        }

        match td.get_node(1).max_is().iter().max_by_key(|(_, obj_val)| **obj_val) {
            Some(x) => *x.1,
            None => {
                eprint!("No obj val at root");
                0
            }
        }
    }

    pub fn retrieve_solutions(td: &TreeDecomposition, graph: &Graph, obj_val: usize) -> String {
        let mut solution = Vec::new();
        let mut rejected = BitVec::from_elem(graph.size(), false);
        td.get_node(1).get_solution(&mut solution, &mut rejected, obj_val, graph, td);

        solution.iter().map(|v| graph.get_vertex_name(*v)).join(",")
    }
}