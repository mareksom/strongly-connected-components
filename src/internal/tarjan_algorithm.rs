use crate::internal::graph::Graph;
use crate::internal::node::Node;
use crate::internal::scc::Scc;
use crate::internal::scc_decomposition::SccDecomposition;

pub struct TarjanAlgorithm<'a> {
    n: usize,
    graph: &'a Graph,
}

#[derive(Copy, Clone)]
enum NodeState {
    Unvisited,
    OnStack,
    Removed,
}

enum DfsStackElement {
    Entry {
        parent_external_id: Option<usize>,
        node_external_id: usize,
    },
    Exit {
        parent_external_id: Option<usize>,
        node_external_id: usize,
        node_internal_id: usize,
    },
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
        let mut dfs_stack: Vec<DfsStackElement> = (0..self.n)
            .map(|node_id| DfsStackElement::Entry {
                parent_external_id: None,
                node_external_id: node_id,
            })
            .collect();
        while let Some(top_element) = dfs_stack.pop() {
            self.process_dfs_stack(&mut state, &mut dfs_stack, top_element);
        }
        self.extract_scc_decomposition(state)
    }

    fn process_dfs_stack(
        &self,
        state: &mut State,
        dfs_stack: &mut Vec<DfsStackElement>,
        element: DfsStackElement,
    ) {
        match element {
            DfsStackElement::Entry {
                parent_external_id,
                node_external_id,
            } => match state.external_node_states[node_external_id] {
                NodeState::Unvisited => {
                    let node_internal_id = state.next_internal_id;
                    state.next_internal_id += 1;
                    state.external_node_states[node_external_id] = NodeState::OnStack;
                    state.stack.push(node_external_id);
                    state.lowlink[node_external_id] = node_internal_id;
                    dfs_stack.push(DfsStackElement::Exit {
                        parent_external_id,
                        node_external_id,
                        node_internal_id,
                    });
                    dfs_stack.extend(self.graph.edges[node_external_id].iter().map(
                        |successor_external_id| DfsStackElement::Entry {
                            parent_external_id: Some(node_external_id),
                            node_external_id: *successor_external_id,
                        },
                    ));
                }
                NodeState::OnStack => {
                    if let Some(parent_id) = parent_external_id {
                        state.lowlink[parent_id] =
                            state.lowlink[parent_id].min(state.lowlink[node_external_id]);
                    }
                }
                NodeState::Removed => (),
            },
            DfsStackElement::Exit {
                parent_external_id,
                node_external_id,
                node_internal_id,
            } => {
                let result = state.lowlink[node_external_id];
                if result == node_internal_id {
                    Self::collect_sccs_from_stack(state, node_external_id);
                }
                if let Some(parent_id) = parent_external_id {
                    state.lowlink[parent_id] = state.lowlink[parent_id].min(result);
                }
            }
        }
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
