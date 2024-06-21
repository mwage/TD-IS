use std::{cmp, collections::HashMap};

use super::{Bag, Graph, TreeDecomposition};


#[derive(Debug)]
pub struct Node {
    bag: Bag,
    node_type: NodeType,
    next: usize,
    prev: Vec<usize>,
    max_is: HashMap<Vec<usize>, usize>
}

impl Node {
    pub fn new(bag: Bag, node_type: NodeType, next: usize) -> Self {
        Node {
            bag,
            node_type,
            next,
            prev: Vec::new(),
            max_is: HashMap::new()
        }
    }

    pub fn next(&self) -> usize {
        self.next
    }

    pub fn prev(&self) -> &Vec<usize> {
        &self.prev
    }

    pub fn add_to_prev(&mut self, prev: usize) {
        self.prev.push(prev)
    }
    
    pub fn max_is(&self) -> &HashMap<Vec<usize>, usize> {
        &self.max_is
    }

    pub fn get_weight(&self, set: &Vec<usize>) -> usize {
        *self.max_is.get(set).unwrap()
    }

    pub fn can_traverse(&self, td: &TreeDecomposition) -> bool {
        match self.node_type {
            NodeType::Join => {
                self.prev().iter().all(|i| !td.get_node(*i).max_is().is_empty())
            },
            NodeType::Root => false,
            _ => true
        }
    }

    pub fn update_entries(&mut self, graph: &Graph, td: &TreeDecomposition) {
        match self.node_type {
            NodeType::Leaf => {
                for set in self.bag.get_powerset() {
                    if graph.is_is(&set) {
                        let weight = graph.get_weight_of_set(&set);
                        self.max_is.insert(set, weight);
                    } else {
                        self.max_is.insert(set, 0);
                    }
                }
                // println!("Updated leaf: {:?}", self);
            },
            NodeType::Forget(v) => {
                for mut set in self.bag.get_powerset() {
                    let prev_node = td.get_node(self.prev[0]);
                    let prev_weight = prev_node.get_weight(&set);
                    set.push(v);    // Add forgotten vertex to set for weight calculation
                    set.sort();
                    let weight = cmp::max(prev_weight, prev_node.get_weight(&set));
                    set.remove(set.iter().position(|x| *x == v).unwrap());    // Remove forgotten vertex again
                    
                    self.max_is.insert(set, weight);
                }

                // println!("Updated forget: {:?}", self);
            },
            NodeType::Introduce(v) => {
                for mut set in self.bag.get_powerset() {
                    let prev_node = td.get_node(self.prev[0]);
                    let weight = if set.contains(&v) {
                        set.remove(set.iter().position(|x| *x == v).unwrap());    // Remove introduced vertex
                        let w = if graph.is_neighbor(v, &set) {
                            0   // Not an IS
                        } else {
                            prev_node.get_weight(&set) + graph.get_weight(v)
                        };
                        set.push(v);
                        set.sort();
                        w
                    } else {
                        prev_node.get_weight(&set)
                    };
                    self.max_is.insert(set, weight);
                }

                // println!("Updated introduce: {:?}", self);
            },
            NodeType::Join => {
                for set in self.bag.get_powerset() {
                    let left_weight = td.get_node(self.prev[0]).get_weight(&set);
                    let right_weight = td.get_node(self.prev[1]).get_weight(&set);
                    let weight = if left_weight == 0 {
                        right_weight
                    } else if right_weight == 0 {
                        left_weight
                    } else {
                        left_weight + right_weight - graph.get_weight_of_set(&set)
                    };
                    self.max_is.insert(set, weight);
                }

                // println!("Updated join: {:?}", self);
            },
            NodeType::Root => {}
        };
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Leaf,
    Introduce(usize),
    Forget(usize),
    Join,
    Root
} 