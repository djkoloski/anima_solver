use crate::{Data, Direction, State, Transition};
use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};

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

pub fn solve(initial_state: State, data: &Data) -> Option<Vec<Direction>> {
    let mut states = HashSet::new();
    let mut parents = Vec::new();
    let mut queue = BinaryHeap::new();

    // Add transitions from initial state
    for (action, transition) in initial_state.transitions(data) {
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
        if !states.contains(&parent_node.state) {
            for (action, transition) in parent_node.state.transitions(data) {
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
            states.insert(parent_node.state);
        }
    }

    None
}
