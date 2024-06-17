use super::Vertices;

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

    pub fn set_vertices(&mut self, vertex_str: &str, vertices: &Vertices) {
        let splits = vertex_str.split(";").collect::<Vec<&str>>();
        if splits.is_empty() { return; }
        
        self.vertices = splits.iter().map(|vs| *vertices.get_idx(vs.trim()).unwrap()).collect();
    }

    pub fn vertices(&self) -> &Vec<usize> {
        &self.vertices
    }
}