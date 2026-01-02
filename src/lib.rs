//! Decomposes a graph into [Strongly Connected Components](https://en.wikipedia.org/wiki/Strongly_connected_component)
//! and to sorts them in [topological order](https://en.wikipedia.org/wiki/Topological_sorting).
//!
//! ## Usage
//!
//! 1. Define the [`Graph`] structure:
//! ```
//! // ┌───┐  ┌───┐  ┌───┐
//! // │ a ├─►│ b ├─►│ c │
//! // └───┘  └─▲─┘  └─┬─┘
//! //          └──────┘
//! # use strongly_connected_components::Node;
//! # use strongly_connected_components::Graph;
//! let mut graph = Graph::new();
//! let a: Node = graph.new_node();
//! let b: Node = graph.new_node();
//! let c: Node = graph.new_node();
//! graph.new_edge(a, b);
//! graph.new_edge(b, c);
//! graph.new_edge(c, b);
//! ```
//! 2. Run the algorithm and create the [`SccDecomposition`]:
//! ```
//! # use strongly_connected_components::Graph;
//! # use strongly_connected_components::SccDecomposition;
//! # let mut graph = Graph::new();
//! # let a = graph.new_node();
//! # let b = graph.new_node();
//! # let c = graph.new_node();
//! # graph.new_edge(a, b);
//! # graph.new_edge(b, c);
//! # graph.new_edge(c, b);
//! let decomp: SccDecomposition = graph.find_sccs();
//! // There are 2 SCCs in the example graph: `[a]` and `[b, c]`.
//! assert_eq!(decomp.len(), 2);
//! ```
//! 3. Check whether two nodes belong to the same [`Scc`]:
//! ```
//! # use strongly_connected_components::Graph;
//! # use strongly_connected_components::Scc;
//! # use strongly_connected_components::SccDecomposition;
//! # let mut graph = Graph::new();
//! # let a = graph.new_node();
//! # let b = graph.new_node();
//! # let c = graph.new_node();
//! # graph.new_edge(a, b);
//! # graph.new_edge(b, c);
//! # graph.new_edge(c, b);
//! # let decomp = graph.find_sccs();
//! let a_scc: &Scc = decomp.scc_of_node(a);
//! let b_scc: &Scc = decomp.scc_of_node(b);
//! let c_scc: &Scc = decomp.scc_of_node(c);
//!
//! // Node `a` belongs to a different SCC than `b` and `c`.
//! assert_ne!(a_scc, b_scc);
//! assert_ne!(a_scc, c_scc);
//!
//! // Nodes `b` and `c` belong to the same SCC.
//! assert_eq!(b_scc, c_scc);
//! ```
//! 4. List all nodes belonging to [`Scc`]:
//! ```
//! # use strongly_connected_components::Node;
//! # use strongly_connected_components::Graph;
//! # let mut graph = Graph::new();
//! # let a = graph.new_node();
//! # let b = graph.new_node();
//! # let c = graph.new_node();
//! # graph.new_edge(a, b);
//! # graph.new_edge(b, c);
//! # graph.new_edge(c, b);
//! # let decomp = graph.find_sccs();
//! # let a_scc = decomp.scc_of_node(a);
//! # let b_scc = decomp.scc_of_node(b);
//! assert_eq!(a_scc.len(), 1);
//! let a_scc_all: Vec<Node> = a_scc.iter_nodes().collect();
//! assert_eq!(a_scc_all, vec![a]);
//!
//! assert_eq!(b_scc.len(), 2);
//! let b_scc_all: Vec<Node> = b_scc.iter_nodes().collect();
//! assert_eq!(b_scc_all, vec![c, b]);
//! ```
//! 5. List [`Scc`]s in topological order:
//! ```
//! # use strongly_connected_components::Node;
//! # use strongly_connected_components::Graph;
//! # use strongly_connected_components::Scc;
//! # use strongly_connected_components::SccDecomposition;
//! # let mut graph = Graph::new();
//! # let a = graph.new_node();
//! # let b = graph.new_node();
//! # let c = graph.new_node();
//! # graph.new_edge(a, b);
//! # graph.new_edge(b, c);
//! # graph.new_edge(c, b);
//! # let decomp = graph.find_sccs();
//! # let a_scc: &Scc = decomp.scc_of_node(a);
//! # let b_scc: &Scc = decomp.scc_of_node(b);
//! let sccs: Vec<&Scc> = decomp.iter_sccs().collect();
//! assert_eq!(sccs, vec![b_scc, a_scc]);
//! ```
//! 6. List [`Node`]s in topological order (with arbitrary order within [`Scc`]s):
//! ```
//! # use strongly_connected_components::Node;
//! # use strongly_connected_components::Graph;
//! # use strongly_connected_components::Scc;
//! # use strongly_connected_components::SccDecomposition;
//! # let mut graph = Graph::new();
//! # let a = graph.new_node();
//! # let b = graph.new_node();
//! # let c = graph.new_node();
//! # graph.new_edge(a, b);
//! # graph.new_edge(b, c);
//! # graph.new_edge(c, b);
//! # let decomp = graph.find_sccs();
//! # let a_scc: &Scc = decomp.scc_of_node(a);
//! # let b_scc: &Scc = decomp.scc_of_node(b);
//! let nodes: Vec<Node> = decomp.iter_nodes().collect();
//! // Either of those would be a valid order,
//! // because `b` and `c` belong to the same SCC.
//! assert!(nodes == vec![c, b, a] || nodes == vec![b, c, a]);
//! ```
mod internal;

pub use internal::graph::Graph;
pub use internal::node::Node;
pub use internal::scc::Scc;
pub use internal::scc_decomposition::SccDecomposition;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph() {
        let graph = Graph::default();
        let decomp = graph.find_sccs();
        assert!(decomp.is_empty());
        assert_eq!(decomp.len(), 0);
    }

    #[test]
    fn single_node_no_edges() {
        let mut graph = Graph::new();
        let node = graph.new_node();
        let decomp = graph.find_sccs();
        assert!(!decomp.is_empty());
        assert_eq!(decomp.len(), 1);
        let scc: Vec<Node> = decomp.iter_nodes().collect();
        assert_eq!(scc, vec![node]);
    }

    #[test]
    fn single_node_and_edges() {
        let mut graph = Graph::new();
        let node = graph.new_node();
        graph.new_edge(node, node);
        graph.new_edge(node, node);
        graph.new_edge(node, node);
        graph.new_edge(node, node);
        let decomp = graph.find_sccs();
        assert!(!decomp.is_empty());
        assert_eq!(decomp.len(), 1);
        let scc: Vec<Node> = decomp.iter_nodes().collect();
        assert_eq!(scc, vec![node]);
    }

    #[test]
    fn separate_nodes() {
        let mut graph = Graph::new();
        let a = graph.new_node();
        let b = graph.new_node();
        let c = graph.new_node();
        graph.new_edge(a, a);
        graph.new_edge(c, c);
        let decomp = graph.find_sccs();
        assert!(!decomp.is_empty());
        assert_eq!(decomp.len(), 3);
        let a_scc: Vec<Node> = decomp.scc_of_node(a).iter_nodes().collect();
        let b_scc: Vec<Node> = decomp.scc_of_node(b).iter_nodes().collect();
        let c_scc: Vec<Node> = decomp.scc_of_node(c).iter_nodes().collect();
        assert_eq!(a_scc, vec![a]);
        assert_eq!(b_scc, vec![b]);
        assert_eq!(c_scc, vec![c]);
    }

    #[test]
    fn manual_example() {
        // Tests the following graph:
        //                 ╔══════════════════════╗
        //                 ║  ┌───┐  ┌───┐  ┌───┐ ║     ╔════════╗
        //        ┌────────╫─►│ 1 ├─►│ 9 ├─►│ 6 │─╫──┐  ║   ┌─┐  ║
        //    ╔═══╪═══╗    ║  └─▲─┘  └───┘  └─┬─┘ ║  │  ║  ┌┴─▼┐ ║
        //    ║ ┌─┴─┐ ║    ║    └─────────────┘   ║  └──╫─►│ 2 │ ║
        //    ║ │ 4 │ ║    ╚══════════════════════╝     ║  └▲─┬┘ ║
        //    ║ └─┬─┘ ║ ╔═════════════════════════════╗ ║   │ │  ║
        //    ╚═══╪═══╝ ║  ┌───┐  ┌───┐  ┌───┐  ┌───┐ ║ ║  ┌┴─▼┐ ║
        //        └─────╫─►│ 0 ├─►│ 3 ├─►│ 7 ├─►│ 8 │─╫─╫─►│ 5 │ ║
        //              ║  └─▲─┘  └┬─▲┘  └───┘  └─┬─┘ ║ ║  └───┘ ║
        //              ║    └─────┘ └────────────┘   ║ ╚════════╝
        //              ╚═════════════════════════════╝
        let mut graph = Graph::new();
        let v0 = graph.new_node();
        let v1 = graph.new_node();
        let v2 = graph.new_node();
        let v3 = graph.new_node();
        let v4 = graph.new_node();
        let v5 = graph.new_node();
        let v6 = graph.new_node();
        let v7 = graph.new_node();
        let v8 = graph.new_node();
        let v9 = graph.new_node();
        graph.new_edge(v4, v1);
        graph.new_edge(v1, v9);
        graph.new_edge(v9, v6);
        graph.new_edge(v6, v1);
        graph.new_edge(v4, v0);
        graph.new_edge(v0, v3);
        graph.new_edge(v3, v0);
        graph.new_edge(v3, v7);
        graph.new_edge(v7, v8);
        graph.new_edge(v8, v3);
        graph.new_edge(v6, v2);
        graph.new_edge(v8, v5);
        graph.new_edge(v2, v2);
        graph.new_edge(v2, v5);
        graph.new_edge(v5, v2);
        let decomp = graph.find_sccs();

        assert_eq!(decomp.len(), 4);

        // Class [4]
        assert_eq!(decomp.scc_of_node(v4).len(), 1);
        let scc_v4: Vec<Node> = decomp.scc_of_node(v4).iter_nodes().collect();
        assert_eq!(scc_v4, vec![v4]);

        // Class [1, 9, 6]
        assert_eq!(decomp.scc_of_node(v1).len(), 3);
        assert_eq!(decomp.scc_of_node(v1), decomp.scc_of_node(v9));
        assert_eq!(decomp.scc_of_node(v1), decomp.scc_of_node(v6));
        let scc_v9: Vec<Node> = decomp.scc_of_node(v9).iter_nodes().collect();
        assert_eq!(scc_v9, vec![v6, v9, v1]);

        // Class [0, 3, 7, 8]
        assert_eq!(decomp.scc_of_node(v0).len(), 4);
        assert_eq!(decomp.scc_of_node(v0), decomp.scc_of_node(v3));
        assert_eq!(decomp.scc_of_node(v0), decomp.scc_of_node(v7));
        assert_eq!(decomp.scc_of_node(v0), decomp.scc_of_node(v8));
        let scc_v8: Vec<Node> = decomp.scc_of_node(v8).iter_nodes().collect();
        assert_eq!(scc_v8, vec![v8, v7, v3, v0]);

        // Class [2, 5]
        assert_eq!(decomp.scc_of_node(v2).len(), 2);
        assert_eq!(decomp.scc_of_node(v2), decomp.scc_of_node(v5));
        let scc_v2: Vec<Node> = decomp.scc_of_node(v2).iter_nodes().collect();
        assert_eq!(scc_v2, vec![v2, v5]);
    }
}
