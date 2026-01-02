use crate::internal::node::Node;

/// Represents a single
/// [Strongly Connected Component](https://en.wikipedia.org/wiki/Strongly_connected_component)
/// (SCC) of a [`Graph`](crate::Graph).
pub struct Scc {
    pub(super) id: usize,
    pub(super) nodes: Vec<Node>,
}

#[allow(clippy::len_without_is_empty)]
impl Scc {
    /// Returns the number of [`Node`]s in this SCC.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns an iterator over all nodes belonging to this SCC.
    /// The nodes are returned in no particular order.
    pub fn iter_nodes(&self) -> impl Iterator<Item = Node> {
        self.nodes.iter().copied()
    }
}

impl std::fmt::Debug for Scc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scc{:?}", self.nodes)
    }
}

impl std::cmp::PartialEq for Scc {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::cmp::Eq for Scc {}

impl std::hash::Hash for Scc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
