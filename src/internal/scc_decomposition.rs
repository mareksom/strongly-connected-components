use crate::internal::node::Node;
use crate::internal::scc::Scc;

/// Represents the decomposition of a [`Graph`](crate::Graph) into
/// [Strongly Connected Components](https://en.wikipedia.org/wiki/Strongly_connected_component)
/// (SCC).
pub struct SccDecomposition {
    pub(super) nodes_to_sccs: Vec<usize>,
    pub(super) sccs: Vec<Scc>,
}

impl SccDecomposition {
    /// Returns the number of SCCs in the decomposition.
    pub fn len(&self) -> usize {
        self.sccs.len()
    }

    /// Returns `true` if there are no SCCs.
    pub fn is_empty(&self) -> bool {
        self.sccs.is_empty()
    }

    /// Returns the SCC containing the given node.
    /// Each node of the [`Graph`](crate::Graph) will always
    /// be assigned to exactly one SCC.
    pub fn scc_of_node(&self, node: Node) -> &Scc {
        assert!(node.id < self.nodes_to_sccs.len());
        let scc_id = self.nodes_to_sccs[node.id];
        assert!(scc_id < self.sccs.len());
        &self.sccs[scc_id]
    }

    /// Returns an iterator over all SCCs in topological order.
    /// If nodes `A` and `B` belong to different SCCs
    /// and the graph has an edge `A->B`, then `B`'s SCC will appear before `A`'s SCC.
    pub fn iter_sccs(&self) -> impl Iterator<Item = &Scc> {
        self.sccs.iter()
    }

    /// Returns an iterator over all nodes in topological order.
    /// If nodes `A` and `B` belong to different SCCs
    /// and the graph has an edge `A->B`, then `B` will appear before `A`.
    pub fn iter_nodes(&self) -> impl Iterator<Item = Node> + '_ {
        self.sccs.iter().flat_map(|sccs| sccs.iter_nodes())
    }
}
