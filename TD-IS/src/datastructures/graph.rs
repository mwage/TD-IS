use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
pub struct Graph {
    vertex_indices: HashMap<String, usize>,
    edges: Vec<Vec<usize>>, // Replace with Bitvec after completing parsing?
    weights: Vec<usize>
}

impl Graph {
    pub fn new(path: &str) -> Self {
        let mut vertex_indices = HashMap::new();
        let mut weights = Vec::new();
        let mut edges = Vec::new();
        for line in read_to_string(path).unwrap().lines() {
            let splits = line.split(",").collect::<Vec<&str>>();            
            if splits.is_empty() { continue; }
            
            let vertex_name = splits[0].trim();
            if vertex_name.is_empty() { continue; } // No empty vertexnames allowed

            if !vertex_indices.contains_key(vertex_name) {
                vertex_indices.insert(vertex_name.to_string(), weights.len());
                weights.push(0);
                edges.push(Vec::new());
            }

            let vertex_idx = *vertex_indices.get(vertex_name).unwrap();

            if splits.len() < 2 { continue; }

            // Edge entry
            let second_vertex_name = splits[1].trim();
            if !second_vertex_name.is_empty() {
                // Add second bag if first occurrence
                if !vertex_indices.contains_key(second_vertex_name) {
                    vertex_indices.insert(second_vertex_name.to_string(), weights.len());
                    weights.push(0);
                    edges.push(Vec::new());
                }
                let second_vertex_idx = *vertex_indices.get(second_vertex_name).unwrap();
                // Add edge
                edges[vertex_idx].push(second_vertex_idx);
                edges[second_vertex_idx].push(vertex_idx);

                continue;   // We don't need edge labels
            }

            if splits.len() < 3 || !splits[1].trim().is_empty() || splits[2].trim().is_empty() { continue; }    // Edge entry, irrelevant for the original graph for us, we only need weights

            match splits[2].parse::<usize>() {
                Ok(res) => weights[vertex_indices[vertex_name]] = res,
                Err(e) => {
                    eprintln!("Label not a positive integer weight: {}", e);                    
                }
            }
        }
        
        Graph {
            vertex_indices,
            weights,
            edges
        }
    }

    pub fn get_vertex_idx(&self, vertex_name: &str) -> Option<&usize> {
        self.vertex_indices.get(vertex_name)
    }

    pub fn get_weight(&self, idx: usize) -> usize {
        self.weights[idx]
    }

    pub fn len(&self) -> usize {
        self.weights.len()
    }

    pub fn edges(&self) -> &Vec<Vec<usize>> {
        &self.edges
    }
}