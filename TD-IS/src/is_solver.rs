use bit_vec::BitVec;

use super::*;


pub struct ISSolver<'a> {
    td: TreeDecomposition,
    graph: &'a Graph
}

impl<'a> ISSolver<'a> {
    fn new(td: TreeDecomposition, graph: &'a Graph) -> Self {
        ISSolver {
            td,
            graph
        }
    }

    pub fn solve(td: TreeDecomposition, graph: &'a Graph) -> usize {
        let solver = Self::new(td, graph);
        solver.compute_is()
    }

    fn compute_is(&self) -> usize {
        let mut finished_leaves = BitVec::from_elem(self.td.leaves().len(), false);
        
        while !finished_leaves.all() {
            let current_leaf = finished_leaves.iter().enumerate().find(|(_, b)| !*b).unwrap().0;
            self.traverse_up(self.td.leaves()[current_leaf]);
            finished_leaves.set(current_leaf, true);
        }

        match self.td.get_node(1).max_is().iter().max_by_key(|(_, obj_val)| **obj_val) {
            Some(x) => *x.1,
            None => {
                eprint!("No obj val at root");
                0
            }
        }
    }

    fn traverse_up(&self, mut current_node: usize) {
        while self.td.get_node(current_node).can_traverse(&self.td) {
            self.td.get_node_mut(current_node).update_entries(&self.graph, &self.td);
            current_node = self.td.get_node(current_node).next();
        }
    }
}