mod vertices;
mod input_tree_decomposition;

use vertices::Vertices;
use input_tree_decomposition::InputTreeDecomposition;

fn main() {
    let vertices = Vertices::new("instances/MiniG.csv");
    let td = InputTreeDecomposition::new("instances/MiniTD.csv", &vertices);
    println!("{:?}", vertices);
    println!("{:?}", td);
}
