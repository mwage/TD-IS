mod datastructures;
mod is_solver;

use datastructures::{TreeDecomposition, Graph};
use is_solver::ISSolver;


fn main() {
    let graph = Graph::new("instances/MiniG.csv");
    let tds = TreeDecomposition::new("instances/MiniTD.csv", &graph);

    // for td in tds.iter() {
    //     println!("{:?}", td);
    // }

    
    let total_obj_val = tds.into_iter().fold(0, |acc, td| acc + ISSolver::solve(td, &graph));

    println!("obj: {}", total_obj_val);
}
