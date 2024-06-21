from sage.graphs.graph import Graph
from sage import graphs
from sage.graphs.graph_decompositions.tree_decomposition import TreelengthConnected
from random import *

def create_rand_graph(num_vertices, density, upper_weight):
    while True:
        graph = Graph(num_vertices)
        for v in range(0, num_vertices):
            w = randrange(0, num_vertices-1)
            if w >= v:
                w += 1
            graph.add_edge(v,w)

        while graph.density() < density:
            v = randrange(0, num_vertices)
            w = randrange(0, num_vertices-1)
            if w >= v:
                w += 1
            graph.add_edge(v,w)

        if graph.is_connected():
            weights = [randint(1,upper_weight) for _ in range(0,num_vertices)]
            return (graph, weights)

def get_graph_str(graph, weights):
    g_str = "\n".join([f"{v},,{weights[v]}" for v in graph.vertices()])
    g_str += "\n"
    g_str += "\n".join([f"{edge[0]},{edge[1]}" for edge in graph.edges()])
    return g_str

def get_td_str(graph):
    td = TreelengthConnected(graph, certificate=True).get_tree_decomposition()
    td_str = ""
    vertices = {}
    for (i, v) in enumerate(td.vertices()):
        vertices[v] = i
        v_names = ";".join([str(x) for x in v])
        td_str += f"N{i},,{v_names}\n"
    td_str += "\n".join([f"N{vertices[edge[0]]},N{vertices[edge[1]]}," for edge in td.edges()])
    return td_str

def create_instance(num_vertices, density, upper_weight, name):
    (g, weights) = create_rand_graph(num_vertices, density, upper_weight)
    g_str = get_graph_str(g, weights)
    td_str = get_td_str(g)

    print(g_str)
    print(td_str)

    with open(f"../TD-IS/instances/{name}G.csv", "w") as text_file:
        text_file.write(g_str)
    with open(f"../TD-IS/instances/{name}TD.csv", "w") as text_file:
        text_file.write(td_str)