use super::Graph;

#[derive(Debug, Clone)]
pub struct Bag {
    vertices: Vec<usize>
}

impl Bag {
    pub fn new_empty() -> Self {
        Bag { 
            vertices: Vec::new()
        }
    }

    pub fn new(vertices: Vec<usize>) -> Self {
        Bag {
            vertices
        }
    }

    pub fn set_vertices(&mut self, vertex_str: &str, graph: &Graph) {
        let splits = vertex_str.split(";").collect::<Vec<&str>>();
        if splits.is_empty() { return; }
        
        self.vertices = splits.iter().map(|vs| *graph.get_vertex_idx(vs.trim()).unwrap()).collect();
    }

    pub fn vertices(&self) -> &Vec<usize> {
        &self.vertices
    }
}