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

Opinion: working with `Rc<RefCell<>>` in Rust feels very off, since it abandons some of the core designs goals of Rust - catching bugs at compile time and introducing runtime cost that other languages avoid. Not to mention the syntax which at time can be daunting. Can we do better? Is it possible to employ graphs that go better with Rust?

## Owning the graph

The problems encountered above arise from the same source: C/C++-like pointer structures don't go well with Rust. Sure, they can be done, but at what cost? Maybe instead of trying to force Rust into doing something it clearly does not want us to do, we should try different approach?

We could reason somewhat like this: if multiple mutable ownership is the problem, maybe let's abandon it altogether? Maybe we could let a single object own the whole graph?

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
    impl Graph {
    pub fn new() -> Graph {
    Graph { nodes: Vec::new(), edges: Vec::new(), highest_idx:0 }
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

This implementation and representations may not be the best one - but it is not meant to be; I simply want to showcase an idea, not implement the fastest graph at mimuw.

This implementations has its problems of algorithmic nature (problematic removal and so on), but, I will write that again, it plays nice with Rust. There is no runtime penalty and we enable compiler to help us. It also is not a syntactic nightmare.

## Hero we definitely need

If one tries to implement graphs in Rust and embarks on Google journey how to exactly do it, they will find that the go-to answer is: it's a nightmare, use Petgraph. It's a sensible route to choose - Petgraph really simplifies things. In fact my team has done exactly that with our final program. Let's dive deeper into how Petgraph does it magic:

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

At first glance it is, but if we learn that `IndexMap` is a crate that essentially provides a map (this crate claims to be compatible with Rust's `HashMap`), this example also is similar to the previous one (in a sense of using arena allocation [region-based memory management]).

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

The point of talking about all this is not to argue that representations X is better in some algorithmic sense than Y; it is to see that pointer based approach, which may work well in other language, is the hard one, which results in a nightmarish code. The proof by Petgraph example shows that real world implementations avoid pointers - for a good reason.

Here's some Petgraph code in action:

{{ include_code_sample(path="lessons/B1_rusty_graphs/petgraph_sample_dfs.rs", language="rust") }}

Sources:

[Petgraph](https://docs.rs/petgraph/latest/petgraph/index.html)

[Petgraph's source code](https://github.com/petgraph/petgraph)

[Rust's dev post about graphs](https://github.com/nrc/r4cppp/blob/master/graphs/README.md)

[Another blog post about graphs](https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)

Author: Mikołaj Piróg
