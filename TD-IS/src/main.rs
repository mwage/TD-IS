use datastructures::{TreeDecomposition, Vertices};

mod datastructures;


fn main() {
    let graph_path = "instances/MiniG.csv";
    let vertices = Vertices::new(graph_path);
    let tds = TreeDecomposition::new("instances/MiniTD.csv", &vertices);
    
    println!("{:?}", tds);
}
