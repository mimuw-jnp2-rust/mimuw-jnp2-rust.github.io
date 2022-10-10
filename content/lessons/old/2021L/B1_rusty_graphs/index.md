+++
title = "[Bonus] Rusty graphs"
date = 2022-06-07 
weight = 1
[extra]
lesson_date = 2022-06-07 
+++

## Introduction

We all had to deal with graphs in Rust, thanks to the task "corpos". Handling the graphs the way that was intended - implementing adjacency list by using `Rc<RefCell<Node>>` - had its drawbacks: an endless stream of derefing, a risk of creating a reference cycle and this mysterious "interior mutability" mechanism.

## Rc madness

To quickly recap why and how this works: `Rc<>` is a smart pointer that utilizes reference counting, similar to `shared_ptr<>` from C++. However, due to Rust's ownerships rules it does not allow us to mutate the data behind the pointer. That's why we don't actually point to the node itself - we hide it behind `RefCell<>` allowing us to mutate the data, by utilizing interior mutability pattern.

There are serval caveats, however. First, we lose compiler's help - it won't scream at us when we try to do something illegal. Speaking technically, when using `RefCell<>` Rust can no longer help us statically - it employs runtime checks to ensure ownership rules. The program will panic if something goes wrong, which is somewhat orthogonal to Rust's mission of catching bugs at compile time. Even if our program is bug-less, we still have to pay performance penalty, since runtime checks are not free.

Opinion: working with `Rc<RefCell<>>` in Rust feels very off, since it abandons some of the core designs goals of Rust - catching bugs at compile time and introducing runtime cost that other languages avoid. Not to mention the syntax which at time can be daunting. Can we do better?

## Owning the graph

The problems encountered above arise from the same source: C/C++-like pointer structures don't go well with Rust. Sure, they can be done, but at what cost? Maybe instead of trying to force Rust into doing something it clearly does not want us to do, we should try different approach?

We could reason somewhat like this: if multiple mutable ownership is the problem, maybe let's abandon it altogether? What about a single object owning the whole graph?

```rust
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    highest_idx: NodeIdx,
}

struct Node {
    idx: NodeIdx,
    data: String,
}

struct Edge {
    idx1: NodeIdx,
    idx2: NodeIdx,
}
```

Here we implement a graph as a single, non-recursive struct, that owns the whole graph. The nodes are identified by their index. This implementation has its disadvantages when compared to adjacency list, but it has one major advantage: it plays nice with Rust as a language.

```rust
pub type NodeIdx = usize;


struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    highest_idx: NodeIdx,
}


struct Node {
    idx: NodeIdx,
    data: String,
}

struct Edge {
    idx1: NodeIdx,
    idx2: NodeIdx,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new(), edges: Vec::new(), highest_idx: 0 }
    }

    pub fn add_node(&mut self, data: &str) {
        self.nodes.push(Node { idx: self.highest_idx + 1, data: data.to_string() });
        self.highest_idx += 1;
    }

    pub fn add_edge(&mut self, idx1: NodeIdx, idx2: NodeIdx) {
        self.edges.push(Edge{idx1, idx2});
    }
}
```

```rust
fn main() {
    let mut graph = Graph::new();

    graph.add_node("123");

    graph.add_edge(0, 0);

}
```

This implementation has its problems of algorithmic nature (problematic removal and so on), but it plays nice with Rust. There is no runtime penalty and we enable compiler to help us. It also is not a syntactic nightmare.

## Real-world solution

If one tries to implement graphs in Rust and embarks on a Google journey how to exactly do it, they will find that the go-to answer is: it's a nightmare, use Petgraph. It's a sensible route to choose - Petgraph really simplifies things. In fact my team has done exactly that with our final program. Let's dive deeper into how Petgraph does its magic:

```rust
pub struct Graph<N, E, Ty = Directed, Ix = DefaultIx> {
    nodes: Vec<Node<N, Ix>>,
    edges: Vec<Edge<E, Ix>>,
    ty: PhantomData<Ty>,
}
```

Huh, this is isomorphic to what's been discussed above. Maybe the next one will be different?

```rust
    pub struct GraphMap<N, E, Ty> {
    nodes: IndexMap<N, Vec<(N, CompactDirection)>>,
    edges: IndexMap<(N, N), E>,
    ty: PhantomData<Ty>,
}
```

At first glance it is, but if we learn that `IndexMap` is a crate that essentially provides a map, we see that this example also is similar to the previous one (in a sense of using arena allocation [region-based memory management]).

One last representation to go:

```rust
pub struct MatrixGraph<N, E, Ty = Directed, Null: Nullable<Wrapped = E> = Option<E>, Ix = DefaultIx>
{
    node_adjacencies: Vec<Null>,
    node_capacity: usize,
    nodes: IdStorage<N>,
    nb_edges: usize,
    ty: PhantomData<Ty>,
    ix: PhantomData<Ix>,
}
```

This representations uses flattened 2D array to store the graph. No pointers in sight.

Of course, those representations are not ideal: they have their issues. To name a few: `Graph` is problematic when it comes to frequent removals of nodes or edges - since they are stored in `Vec`, one has to either put some kind of placeholder in place of node/edge (which is not very Rusty) or copy the whole array (which is slow). `GraphMap` mitigates this issues by using hash map to store the graph, but that introduces some requirements on the `Node` type: it must implement `Ord` as well as `Copy` and `Eq + Hash` - which may be suitable for integers, but not exactly for more complicated types. `MatrixGraph` takes up much space and, similarly to `Graph`, has its problems when it comes to removing nodes.

Yet, all this problems are not Rusty in nature - they hold across every programming language.

The point of talking about all this is not to argue that representations X is better in some algorithmic sense than Y; it is to see that pointer based approach, which may work well in other language, is the hard one in Rust and results in a nightmarish code. The proof by Petgraph example shows that real world implementations avoid pointers - for a good reason.

Here's some Petgraph code in action:

```rust
fn dfs_helper(&mut self, graph: &mut Graph, node_index: NodeIndex) {
    self.add_step(AlgorithmStep::Node(NodeStep::new(
        node_index,
        NodeState::Queued,
    )));

    if let Some(node) = graph.node_weight_mut(node_index) {
        node.set_state(NodeState::Queued)
    }

    let mut walker = graph
        .neighbors_directed(node_index, Direction::Outgoing)
        .detach();

    while let Some((edge_idx, other_node_idx)) = walker.next(graph) {
        if let Some(other_state) = graph
            .node_weight(other_node_idx)
            .map(|node| node.get_state())
        {
            if matches!(other_state, NodeState::NotVisited) {
                self.add_step(AlgorithmStep::Edge(EdgeStep::new(edge_idx)));
                self.dfs_helper(graph, other_node_idx);
            }
        }
    }

    self.add_step(AlgorithmStep::Node(NodeStep::new(
        node_index,
        NodeState::Visited,
    )));

    if let Some(node) = graph.node_weight_mut(node_index) {
        node.set_state(NodeState::Visited)
    }
}
```

Sources:

[Petgraph](https://docs.rs/petgraph/latest/petgraph/index.html)

[Petgraph's source code](https://github.com/petgraph/petgraph)

[Rust's dev post about graphs](https://github.com/nrc/r4cppp/blob/master/graphs/README.md)

[Another blog post about graphs](https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)

Author: Mikołaj Piróg
