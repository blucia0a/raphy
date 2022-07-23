# raphy 
## A graph data structure library with multiple implementations.
Brandon Lucia -- 2020


## raphy::fast_csr::FastCSR - A fast sparse graph data structure

FastCSR is a better version of the CSR data structure that uses mmapped files, instead
of using text and large in-memory buffers.

The implementation is 100% zero-copy, meaning that it is possible to load up and
traverse a graph in CSR format without copying the contents from the file to
in-memory buffers.  The consequence of this design is that loading extremely
large graphs is very fast because virtual memory lazily pages graph data into memory
as your traversal needs it, and otherwise the data just sit in the file underlying
the graph.


## raphy::csr::CSR - A Compressed Sparse Row (CSR) graph data structure
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

### So how do you use a CSR & FastCSR?

The FastCSR expects a binary file format containing 2 8-byte values which are
the number of vertices and the number of edges.  Then the file contains a
sequence of 8-byte values, one value per vertex.  Then the file contains a
sequence of 8-byte values, one value per edge.  The structure's meaning is exactly
the CSR that it stores:  the first sequence of 8-byte values is the offsets array
and the second sequence of 8-byte values is the neighbors array.

You can directly load a CSR in this format using the `FastCSR::new(String)` function, the 
argument to which is a file name string.

From there, you can use `neighbor_scan(f: impl Fn(usize,&[usize]))` to traverse
your graph.  This function takes a closure that you provide.  Your closure
should accept a single usize argument, which is a source vertex id, and a slice
reference of usizes, which are the vertex ids of vertices adjacent to the
source vertex.

As an alternative, you can use `read_only_scan(f: impl Fn(usize,usize))` to traverse
your graph.  This function takes a closure that you provide.  Your closure should accept
two usize arguments, which are the source and destination vertices of an edge.
The function gets called for each edge in your graph.

Here is an example program that implements something like the PageRank algorithm:

```
fn main() {

    let fcsr = FastCSR::new(String::from("./large.csr"));

    let mut vp1 = Vec::with_capacity(fcsr.getv());
    for _ in 0..fcsr.getv() {
        vp1.push(RwLock::new(0.0));
    }

    for _ in 0..10 {
        let mut vp2 = Vec::with_capacity(fcsr.getv());
        for _ in 0..fcsr.getv() {
            vp2.push(RwLock::new(0.0));
        }

        let vf = |v: usize, nei: &[usize]| {
            const D: f64 = 0.85;
            let mut n_upd: f64 = 0.0;

            nei.iter()
                .for_each(|v1| n_upd = n_upd + *vp1[*v1].read().unwrap() / (nei.len() as f64));

            {
                let mut prop = vp2[v].write().unwrap();
                *prop = (1.0 - D) / (fcsr.getv() as f64) + D * n_upd;
            }
        };

        fcsr.neighbor_scan(vf);
        for v in 0..(vp1.len() - 1) {
            *vp1[v].write().unwrap() = *vp2[v].read().unwrap();
        }
    }

    vp1.iter().enumerate().for_each(|(i, v)| {
        println!("{} {}", i, *v.read().unwrap());
    });
    
}
```


### So what if you only have an edge list?

The CSR module now contains support to create a CSR from a binary file
formatted edge list.   The binary edge list format is a sequence of pairs
of 8-byte values.  Each pair of 8-byte values represents a single edge.

To create a CSR from a binary edge list, you call `new_from_el_mmap(v: usize, f: String)`.
The first argument to this function is the number of vertices in your graph, which you
have to provide up front.  It is possible to get this from scanning the graph once, but the
cost is relatively high, to do an order |E| traversal to compute |V|, so the API requires
this parameter.  The second argument is a filename string containing the binary edge list.

An example program to generate a FastCSR from a binary edge list would do this:

```
fn main() {

    let csr = CSR::new_from_el_mmap(10000000,String::from("large.el"));

    csr.write_fastcsr(String::from("large.csr"));
    
}
```

## raphy::graph::Graph - A basic graph data structure
Graphs have vertices that have a numeric identifier and polymorphically can
carry any payload / value type that is displayable and orderable (see the VtxTrait definition).

The examples not prefixed with `csr_` are a good reference for the types of
things that you can do with a graph.  The BFS and DFS examples show how to
traverse a graph's vertices.  The rand_graph and iter_graph example show how to
scan over the vertices of a graph, ignoring graph structure.


### Edge List file format (for reading in edge lists in raphy::csr::CSR)

The format is really simple and intentionally human readable for now:

v0,v1<br/>
v0,v2<br/>
v1,v0<br/>
v1,v2<br/>
v2,v0<br/>
v2,v1<br/>
...

The csr_rand_graph example is written with a scan closure that writes out an edge
list in the correct format.  You can create a test input file by running 
the csr_rand_graph example and putting its output into a file.


### TODO
* ~~Get rid of weights in CSR~~
* ~~Add edge list type to CSR~~ (not doing)
* ~~Add random edge list generator~~
* ~~Add file reading for edge list loading~~
* bit-vec support for frontier and visited in BFS 
* propagation blocking for CSR construction
* propagation blocking for arbitrary traversals
* ~~benchmarks for performance comparisons (vs. C implementation)~~
* kick tires with more algo impls that use the traversal routines
* ~~parallel CSR construction~~
* ~~optimize file reading for large graphs~~
