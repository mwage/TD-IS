mod datastructures;
mod is_solver;

use std::env;

use datastructures::{TreeDecomposition, Graph};
use is_solver::ISSolver;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Paths to input graph and input TD not specified!");
    }
    let graph = Graph::new(&args[1]);
    let tds = TreeDecomposition::new(&args[2], &graph);
    let total_obj_val = tds.iter().fold(0, |acc, td| acc + ISSolver::solve(td, &graph));
    let solution = ISSolver::retrieve_solutions(&tds[0], &graph, total_obj_val);

    println!("obj: {}\n{}", total_obj_val, solution);
}
