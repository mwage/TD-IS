mod datastructures;
mod is_solver;

use datastructures::{TreeDecomposition, Graph};
use is_solver::ISSolver;


fn main() {
    let graph = Graph::new("instances/MiniG.csv");
    let tds = TreeDecomposition::new("instances/MiniTD.csv", &graph);

    let total_obj_val = tds.iter().fold(0, |acc, td| acc + ISSolver::solve(td, &graph));
    let solution = ISSolver::retrieve_solutions(&tds[0], &graph, total_obj_val);
    
    println!("obj: {}\n{}", total_obj_val, solution);
}
