use std::fs::read_to_string;

use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Graph {
    vertex_indices: FxHashMap<String, usize>,
    vertex_names: FxHashMap<usize, String>,
    edges: Vec<Vec<usize>>, // Replace with Bitvec after completing parsing?
    weights: Vec<usize>
}

impl Graph {
    pub fn new(path: &str) -> Self {
        eprintln!("Parsing graph from {}", path);
        let mut vertex_indices = FxHashMap::default();
        let mut vertex_names = FxHashMap::default();
        let mut weights = Vec::new();
        let mut edges = Vec::new();
        for line in read_to_string(path).unwrap().lines() {
            let splits = line.split(",").collect::<Vec<&str>>();            
            if splits.is_empty() { continue; }
            
            let vertex_name = splits[0].trim();
            if vertex_name.is_empty() { continue; } // No empty vertexnames allowed

            if !vertex_indices.contains_key(vertex_name) {
                vertex_indices.insert(vertex_name.to_string(), weights.len());
                vertex_names.insert(weights.len(), vertex_name.to_string());
                weights.push(0);
                edges.push(Vec::new());
            }

            let vertex_idx = *vertex_indices.get(vertex_name).unwrap();

            if splits.len() < 2 { continue; }

            // Edge entry
            let second_vertex_name = splits[1].trim();
            if !second_vertex_name.is_empty() {
                // Add second vertex if first occurrence
                if !vertex_indices.contains_key(second_vertex_name) {
                    vertex_indices.insert(second_vertex_name.to_string(), weights.len());
                    vertex_names.insert(weights.len(), second_vertex_name.to_string());
                    weights.push(0);
                    edges.push(Vec::new());
                }
                let second_vertex_idx = *vertex_indices.get(second_vertex_name).unwrap();
                // Add edge
                edges[vertex_idx].push(second_vertex_idx);
                edges[second_vertex_idx].push(vertex_idx);

                continue;   // We don't need edge labels
            }

            if splits.len() < 3 || splits[2].trim().is_empty() { continue; }

            match splits[2].parse::<usize>() {
                Ok(res) => weights[vertex_indices[vertex_name]] = res,
                Err(e) => {
                    eprintln!("Label not a positive integer weight: {}", e);                    
                }
            }
        }
        
        Graph {
            vertex_indices,
            vertex_names,
            weights,
            edges
        }
    }

    pub fn size(&self) -> usize {
        self.weights.len()
    }

    pub fn get_vertex_idx(&self, vertex_name: &str) -> Option<&usize> {
        self.vertex_indices.get(vertex_name)
    }

    pub fn get_vertex_name(&self, idx: usize) -> &String {
        self.vertex_names.get(&idx).unwrap()
    }

    pub fn get_weight(&self, idx: usize) -> usize {
        self.weights[idx]
    }

    pub fn get_weight_of_set(&self, set: &Vec<usize>) -> usize {
        set.iter().fold(0, |acc, i| acc + self.weights[*i])
    }

    pub fn is_neighbor(&self, vertex: usize, others: &Vec<usize>) -> bool {
        self.edges[vertex].iter().any(|x| others.contains(x))
    }

    pub fn is_is(&self, vertices: &Vec<usize>) -> bool {
        vertices.iter().all(|x| !self.is_neighbor(*x, vertices))
    }
}