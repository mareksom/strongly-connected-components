# strongly-connected-components

[![Crates.io](https://img.shields.io/crates/v/strongly-connected-components.svg)](https://crates.io/crates/strongly-connected-components)
[![Documentation](https://docs.rs/strongly-connected-components/badge.svg)](https://docs.rs/strongly-connected-components)
[![License](https://img.shields.io/crates/l/strongly-connected-components.svg)](https://github.com/mareksom/strongly-connected-components/blob/main/LICENSE)
[![Downloads](https://img.shields.io/crates/d/strongly-connected-components.svg)](https://crates.io/crates/strongly-connected-components)

Decomposes a graph into [Strongly Connected Components](https://en.wikipedia.org/wiki/Strongly_connected_component) and sorts them in [topological order](https://en.wikipedia.org/wiki/Topological_sorting).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
strongly-connected-components = "0.1"
```

## Usage

### 1. Define the graph structure

```rust
// ┌───┐  ┌───┐  ┌───┐
// │ a ├─►│ b ├─►│ c │
// └───┘  └─▲─┘  └─┬─┘
//          └──────┘
let mut graph = Graph::new();
let a: Node = graph.new_node();
let b: Node = graph.new_node();
let c: Node = graph.new_node();
graph.new_edge(a, b);
graph.new_edge(b, c);
graph.new_edge(c, b);
```

### 2. Run the algorithm and create the SccDecomposition

```rust
let decomp: SccDecomposition = graph.find_sccs();
// There are 2 SCCs in the example graph: `[a]` and `[b, c]`.
assert_eq!(decomp.len(), 2);
```

### 3. Check whether two nodes belong to the same SCC

```rust
let a_scc: &Scc = decomp.scc_of_node(a);
let b_scc: &Scc = decomp.scc_of_node(b);
let c_scc: &Scc = decomp.scc_of_node(c);

// Node `a` belongs to a different SCC than `b` and `c`.
assert_ne!(a_scc, b_scc);
assert_ne!(a_scc, c_scc);

// Nodes `b` and `c` belong to the same SCC.
assert_eq!(b_scc, c_scc);
```

### 4. List all nodes belonging to an SCC

```rust
assert_eq!(a_scc.len(), 1);
let a_scc_all: Vec<Node> = a_scc.iter_nodes().collect();
assert_eq!(a_scc_all, vec![a]);

assert_eq!(b_scc.len(), 2);
let b_scc_all: Vec<Node> = b_scc.iter_nodes().collect();
assert_eq!(b_scc_all, vec![c, b]);
```

### 5. List SCCs in topological order

```rust
let sccs: Vec<&Scc> = decomp.iter_sccs().collect();
assert_eq!(sccs, vec![b_scc, a_scc]);
```

### 6. List nodes in topological order

```rust
let nodes: Vec<Node> = decomp.iter_nodes().collect();
// Either of those would be a valid order,
// because `b` and `c` belong to the same SCC.
assert!(nodes == vec![c, b, a] || nodes == vec![b, c, a]);
```

## API Overview

| Type | Description |
|------|-------------|
| `Graph` | The graph structure. Create nodes with `new_node()` and edges with `new_edge()`. |
| `Node` | A node in the graph. |
| `Scc` | A single strongly connected component. Iterate over nodes with `iter_nodes()`. |
| `SccDecomposition` | The result of running `graph.find_sccs()`. Query SCCs and iterate in topological order. |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
