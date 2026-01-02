/// Represents a single node of the graph.
/// A node can be created by calling [`Graph::new_node()`](crate::Graph::new_node).
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Node {
    pub(super) id: usize,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({})", self.id)
    }
}
