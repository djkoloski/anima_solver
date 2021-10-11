use crate::{Data, Direction, State, Transition};
use std::collections::{HashSet, VecDeque};

pub fn solve(initial_state: State, data: &Data) -> Option<Vec<Direction>> {
    let mut states = HashSet::new();
    let mut parents = Vec::new();
    let mut queue = VecDeque::new();
    let mut index = 0;

    // Add transitions from initial state
    for (action, transition) in initial_state.transitions(data) {
        match transition {
            Transition::Indeterminate(state) => {
                parents.push((0, action));
                queue.push_back(state);
            }
            Transition::Success => return Some(vec![action]),
        }
    }

    // Pop states in order
    while let Some(parent_node) = queue.pop_front() {
        index += 1;

        if !states.contains(&parent_node) {
            for (action, transition) in parent_node.transitions(data) {
                match transition {
                    Transition::Indeterminate(state) => {
                        parents.push((index, action));
                        queue.push_back(state);
                    }
                    Transition::Success => {
                        let mut result_actions = vec![action];
                        let mut current_index = index;
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
            states.insert(parent_node);
        }
    }

    None
}
