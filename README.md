# raphy -- Brandon Lucia -- 2020
### A graph data structure library with multiple implementations.

### raphy::graph::Graph - A basic graph data structure
Graphs have vertices that have a numeric identifier and polymorphically can
carry any payload / value type that is displayable and orderable (see the VtxTrait definition).

The examples not prefixed with `csr_` are a good reference for the types of
things that you can do with a graph.  The BFS and DFS examples show how to
traverse a graph's vertices.  The rand_graph and iter_graph example show how to
scan over the vertices of a graph, ignoring graph structure.

### raphy::csr::CSR - A Compressed Sparse Row (CSR) graph data structure
CSR is an optimized graph data representation especially amenable for use with
sparse graph data.  A sparse graph is one with lots of zeroes in its adjacency
matrix.  The operations on a CSR are somewhat limited.  The current CSR
implementation offers read-only traversal and a somewhat rigid page_rank
implementation.

_CSR is currently an experimental feature and I do not recommend using it yet_
