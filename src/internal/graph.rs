use crate::internal::node::Node;
use crate::internal::scc_decomposition::SccDecomposition;
use crate::internal::tarjan_algorithm::TarjanAlgorithm;

/// Represents the graph on which SCC decomposition will be done.
/// Consists of [`Node`]s, connected by directed edges.
///
/// ```
/// # use strongly_connected_components::Graph;
/// let mut graph = Graph::new();
/// let v0 = graph.new_node();
/// let v1 = graph.new_node();
/// let v2 = graph.new_node();
/// let v3 = graph.new_node();
/// graph.new_edge(v0, v1);
/// graph.new_edge(v1, v3);
/// graph.new_edge(v1, v1);
/// // Graph looks like this:
/// // ┌───┐  ┌───┐  ┌───┐ ┌───┐
/// // │ 0 ├─►│ 1 ├─►│ 3 │ │ 2 │
/// // └───┘  └▲─┬┘  └───┘ └───┘
/// //         └─┘
/// ```
#[derive(Clone, Default)]
pub struct Graph {
    pub(super) edges: Vec<Vec<usize>>,
}

impl Graph {
    /// Creates an empty graph with no nodes and no edges.
    /// [`Node`]s can be added by calling [`Graph::new_node`].
    /// Edges can be added by calling [`Graph::new_edge`].
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    /// Creates a new node of the graph with no incoming/outgoing edges.
    /// Edges can be added by calling [`Graph::new_edge`].
    pub fn new_node(&mut self) -> Node {
        let node = Node {
            id: self.edges.len(),
        };
        self.edges.push(Vec::new());
        node
    }

    /// Creates a new directed edge between two nodes.
    pub fn new_edge(&mut self, from: Node, to: Node) {
        assert!(from.id < self.edges.len());
        assert!(to.id < self.edges.len());
        self.edges[from.id].push(to.id);
    }

    /// Computes the Strongly Connected Components decomposition of this graph.
    /// Uses [Tarjan's algorithm](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm)
    /// which runs in O(|N| + |E|) time and uses O(|N| + |E|) memory.
    pub fn find_sccs(&self) -> SccDecomposition {
        TarjanAlgorithm::new(self).solve()
    }
}
