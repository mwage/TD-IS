use std::fs::read_to_string;

use super::{Bag, Graph};
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct InputTreeDecomposition {
    bags: Vec<Bag>,
    edges: Vec<Vec<usize>>
}

impl InputTreeDecomposition {
    pub fn new(path: &str, graph: &Graph) -> Self {
        eprintln!("Parsing treedecomposition from {}", path);
        let mut bag_indices = FxHashMap::default();
        let mut bags = Vec::new();
        let mut edges = Vec::new();
        for line in read_to_string(path).unwrap().lines() {
            let splits = line.split(",").collect::<Vec<&str>>();
            if splits.is_empty() { continue; }
            
            let bag_name = splits[0].trim();
            if bag_name.is_empty() { continue; }    // No empty bag names allowed

            // Add bag if first occurrence
            if !bag_indices.contains_key(bag_name) {
                bag_indices.insert(bag_name.to_string(), bags.len());
                bags.push(Bag::new_empty());
                edges.push(Vec::new());
            }

            let bag_idx = *bag_indices.get(bag_name).unwrap();

            if splits.len() < 2 { continue; }

            // Edge entry
            let second_bag_name = splits[1].trim();
            if !second_bag_name.is_empty() {
                // Add second bag if first occurrence
                if !bag_indices.contains_key(second_bag_name) {
                    bag_indices.insert(second_bag_name.to_string(), bags.len());
                    bags.push(Bag::new_empty());
                    edges.push(Vec::new());
                }
                let second_bag_idx = *bag_indices.get(second_bag_name).unwrap();
                // Add edge
                edges[bag_idx].push(second_bag_idx);
                edges[second_bag_idx].push(bag_idx);

                continue;   // We don't need edge labels
            }
            
            if splits.len() < 3 || splits[2].trim().is_empty() { continue; }    // Empty bag?

            bags[bag_idx].set_vertices(splits[2], graph)
        }
        
        InputTreeDecomposition {
            bags,
            edges
        }
    }

    pub fn edges(&self) -> &Vec<Vec<usize>> {
        &self.edges
    }

    pub fn get_bag(&self, idx: usize) -> &Bag {
        &self.bags[idx]
    }
    
    pub fn len(&self) -> usize {
        self.bags.len()
    }
}
