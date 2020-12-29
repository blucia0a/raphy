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
matrix.  The structure has three arrays, the offsets array, the neighbs array,
and the vtxprop array.  Offsets is indexed by a source vertex's id and an entry stores the 
start index in the neighbs array at which the source vertex's neighbors reside.  
Vertex i's neighbors (and weights) are stored in tuples (v,w) in neighbs[ offsets[i] ] through neighbs[ offsets[i+1] ];
Neighbs is indexed by values grabbed from offsets, listing a vertex's adjacencies.
vtxprop is an auxiliary array storing a vertex property, one per vertex.  Currently,
this property is a f64, but it should really be generic and will be eventually.

The current CSR implementation offers a scan over edges and a BFS traversal
over vertices.  The way you use these scans is to pass in a FnMut that gets to
see each edge or vertex as it gets traversed.  The edge scan FnMut takes in
(v0,v1,w), which are the source, destination, and weight of an edge.  The bfs
traversal FnMut takes in a vertex, v, only.  The bfs_traversal API also takes
the id: usize of a starting vertex.

Reading the example code in examples/csr_*.rs
is the best way to learn how exactly to use these API functions.  

### TODO
* Get rid of weights in CSR
* Add edge list type to CSR
* Add memory mapping for edge list & CSR loading and saving
* Propagation blocking for CSR construction
* Propagation blocking for arbitrary traversals
* Performance comparisons vs. C implementation?
* Kick tires with more algo impls that use the traversal routines

