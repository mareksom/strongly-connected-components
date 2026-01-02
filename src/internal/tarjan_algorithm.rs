use crate::internal::graph::Graph;
use crate::internal::node::Node;
use crate::internal::scc::Scc;
use crate::internal::scc_decomposition::SccDecomposition;

#[derive(Copy, Clone)]
enum NodeState {
    Unvisited,
    OnStack,
    Removed,
}

pub struct TarjanAlgorithm<'a> {
    n: usize,
    graph: &'a Graph,
}

struct State {
    stack: Vec<usize>,
    external_node_states: Vec<NodeState>,
    lowlink: Vec<usize>,
    next_internal_id: usize,
    sccs: Vec<Scc>,
}

impl<'a> TarjanAlgorithm<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let n = graph.edges.len();
        Self { n, graph }
    }

    pub fn solve(self) -> SccDecomposition {
        let mut state = State {
            stack: Vec::with_capacity(self.n),
            external_node_states: vec![NodeState::Unvisited; self.n],
            lowlink: vec![0; self.n],
            next_internal_id: 0,
            sccs: Vec::new(),
        };
        for node_external_id in 0..self.n {
            self.dfs(&mut state, node_external_id);
            assert!(matches!(
                state.external_node_states[node_external_id],
                NodeState::Removed
            ));
        }
        self.extract_scc_decomposition(state)
    }

    fn dfs(&self, state: &mut State, node_external_id: usize) -> usize {
        match state.external_node_states[node_external_id] {
            NodeState::Unvisited => self.visit(state, node_external_id),
            NodeState::OnStack => state.lowlink[node_external_id],
            NodeState::Removed => usize::MAX,
        }
    }

    fn visit(&self, state: &mut State, node_external_id: usize) -> usize {
        let node_internal_id = state.next_internal_id;
        state.next_internal_id += 1;
        state.external_node_states[node_external_id] = NodeState::OnStack;
        state.stack.push(node_external_id);
        state.lowlink[node_external_id] = node_internal_id;
        let min_neighbour_lowlink: usize = self.graph.edges[node_external_id]
            .iter()
            .map(|neighbour_external_id| self.dfs(state, *neighbour_external_id))
            .min()
            .unwrap_or(node_internal_id);
        let result = min_neighbour_lowlink.min(node_internal_id);
        state.lowlink[node_external_id] = result;
        if result == node_internal_id {
            Self::collect_sccs_from_stack(state, node_external_id);
        }
        result
    }

    fn collect_sccs_from_stack(state: &mut State, lowest_node_id: usize) {
        let mut new_scc = Scc {
            id: state.sccs.len(),
            nodes: Vec::new(),
        };
        loop {
            let node_id = state
                .stack
                .pop()
                .expect("assertion: the stack can never be empty here");
            assert!(matches!(
                state.external_node_states[node_id],
                NodeState::OnStack
            ));
            state.external_node_states[node_id] = NodeState::Removed;
            new_scc.nodes.push(Node { id: node_id });
            if node_id == lowest_node_id {
                break;
            }
        }
        state.sccs.push(new_scc);
    }

    fn extract_scc_decomposition(&self, state: State) -> SccDecomposition {
        let mut nodes_to_sccs = vec![0; self.n];
        for scc in &state.sccs {
            for node in &scc.nodes {
                nodes_to_sccs[node.id] = scc.id;
            }
        }
        SccDecomposition {
            nodes_to_sccs,
            sccs: state.sccs,
        }
    }
}
