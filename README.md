# TD-IS
Maximum Independent Set solver for Tree Decompositions (based on Dynamic Programming)

TD-IS contains a maximum independant set (IS) solver which takes a graph and a tree decomposition (TD) as input and computes a nice TD before solving the maxiumum IS problem.
Finally, it outputs the weight of the maximal IS as well as retrieves one such solution, i.e. the set of vertices that make up an IS of maximal weight.

In addition, graph_gen contains a python script that can be used to generate instances to test the solver. Upon providing a number of vertices, minimal density and an upper limit for the vertex weights it generates a random graph with the given properties as well as computes a TD using the Sage library. Finally, it outputs stores the graph as well as the TD to a user specified path ({path}G.csv and {path}TD.csv ) respectively.
A set of benchmark instances of varying parameters is provided in TD-IS/instances.

Usage example:
```
cd TD-IS
cargo run --release -- ./instances/100_1_0G.csv ./instances/100_1_0TD.csv

```