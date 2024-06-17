use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
pub struct Vertices {
    vertices: HashMap<String, usize>,
    weights: Vec<usize>
}

impl Vertices {
    pub fn new(path: &str) -> Self {
        let mut vertices = HashMap::new();
        let mut weights = Vec::new();
        for line in read_to_string(path).unwrap().lines() {
            let splits = line.split(",").collect::<Vec<&str>>();
            
            if splits.is_empty() { continue; }
            
            let vertex_name = splits[0].trim();
            if vertex_name.is_empty() { continue; } // No empty vertexnames allowed

            if !vertices.contains_key(vertex_name) {
                vertices.insert(vertex_name.to_string(), weights.len());
                weights.push(0);
            }
            
            if splits.len() < 3 || !splits[1].trim().is_empty() || splits[2].trim().is_empty() { continue; }    // Edge entry, irrelevant for the original graph for us, we only need weights

            match splits[2].parse::<usize>() {
                Ok(res) => weights[vertices[vertex_name]] = res,
                Err(e) => {
                    eprintln!("Label not a positive integer weight: {}", e);                    
                }
            }
        }
        
        Vertices {
            vertices,
            weights
        }
    }

    pub fn get_idx(&self, vertex_name: &str) -> Option<&usize> {
        self.vertices.get(vertex_name)
    }
}