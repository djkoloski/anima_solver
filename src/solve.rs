use crate::{Data, Direction, State, Transition};
use rustc_hash::FxHasher;
use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    hash::BuildHasher,
};

#[derive(Eq, PartialEq)]
struct Node {
    state: State,
    distance: usize,
    estimate: usize,
    index: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate.cmp(&self.estimate)
    }
}

struct FastHashBuilder;

impl BuildHasher for FastHashBuilder {
    type Hasher = FxHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FxHasher::default()
    }
}

pub fn solve(initial_state: State, data: &Data) -> Option<Vec<Direction>> {
    let mut states = HashMap::with_capacity_and_hasher(4 * 1024, FastHashBuilder);
    let mut parents = Vec::with_capacity(4 * 1024);
    let mut queue = BinaryHeap::with_capacity(1024);

    // Insert initial state
    let initial_transitions = initial_state.transitions(data);
    states.insert(initial_state, ());

    // Add transitions from initial state
    for (action, transition) in initial_transitions {
        match transition {
            Transition::Indeterminate(state) => {
                parents.push((0, action));

                let estimate = state.heuristic(data) + 1;
                queue.push(Node {
                    state,
                    distance: 1,
                    estimate,
                    index: parents.len(),
                });
            }
            Transition::Success => return Some(vec![action]),
        }
    }

    // Pop states in order
    while let Some(parent_node) = queue.pop() {
        if let Entry::Vacant(entry) = states.entry(parent_node.state) {
            for (action, transition) in entry.key().transitions(data) {
                match transition {
                    Transition::Indeterminate(state) => {
                        parents.push((parent_node.index, action));

                        let estimate = state.heuristic(data) + (parent_node.distance + 1);
                        queue.push(Node {
                            state: state,
                            distance: parent_node.distance + 1,
                            estimate,
                            index: parents.len(),
                        });
                    }
                    Transition::Success => {
                        let mut result_actions = vec![action];
                        let mut current_index = parent_node.index;
                        while current_index != 0 {
                            let (next_index, action) = parents.swap_remove(current_index - 1);
                            result_actions.push(action);
                            current_index = next_index;
                        }
                        result_actions.reverse();
                        // println!("Explored {} states", parents.len());
                        return Some(result_actions);
                    }
                }
            }
            entry.insert(());
        }
    }

    None
}
