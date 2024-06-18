use datastructures::{TreeDecomposition, Graph};

mod datastructures;


fn main() {
    let graph_path = "instances/MiniG.csv";
    let vertices = Graph::new(graph_path);
    let tds = TreeDecomposition::new("instances/MiniTD.csv", &vertices);
    
    println!("{:?}", tds);
}
