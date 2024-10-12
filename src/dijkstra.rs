use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::coords::System;
use crate::coords::{euclidean, get_coords, get_neighbours};

#[derive(Clone)]
struct State {
    cost: f64,
    system_id: i64,
}

// Implementing PartialEq and Ord manually so we don't need Eq for f64
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.system_id == other.system_id
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order so BinaryHeap behaves like a min-heap
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn naive_dijkstra(
    start_system: i64,
    goal_system: i64,
    map_data: &[System],
) -> (HashMap<i64, f64>, Vec<i64>) {
    let jump_distance = 60.0;
    let mut epoch = 0;

    let mut priority_queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut costs = HashMap::new();
    let mut predecessors = HashMap::new(); // To track the predecessor of each system

    costs.insert(start_system, 0.0);
    priority_queue.push(State {
        cost: 0.0,
        system_id: start_system,
    });

    while let Some(State {
        cost: curr_cost,
        system_id: curr_node,
    }) = priority_queue.pop()
    {
        if visited.contains(&curr_node) {
            continue;
        }

        visited.insert(curr_node);

        // Debugging information
        println!("Current Epoch:{}", epoch);
        println!("Systems Visited: {}", visited.len());
        println!("Queued Systems: {}", priority_queue.len());
        println!("Cost {}", costs[&curr_node]);

        if curr_node == goal_system {
            break;
        }

        let neighbours = get_neighbours(curr_node, jump_distance, map_data);
        for (neighbour_id, neighbour_coords) in neighbours {
            if !visited.contains(&neighbour_id) {
                let new_cost =
                    curr_cost + euclidean(get_coords(curr_node, map_data), neighbour_coords);

                if !costs.contains_key(&neighbour_id) || new_cost < costs[&neighbour_id] {
                    costs.insert(neighbour_id, new_cost);
                    predecessors.insert(neighbour_id, curr_node); // Update predecessor
                    priority_queue.push(State {
                        cost: new_cost,
                        system_id: neighbour_id,
                    });
                }
            }
        }

        epoch += 1;
        if epoch % 100 == 0 {
            println!("{:?}", costs);
        }
    }

    let path = reconstruct_path(&predecessors, start_system, goal_system);

    (costs, path)
}

// Function to reconstruct the path from start to goal using the predecessor map
fn reconstruct_path(
    predecessors: &HashMap<i64, i64>,
    start_system: i64,
    goal_system: i64,
) -> Vec<i64> {
    let mut path = Vec::new();
    let mut current_system = goal_system;

    while current_system != start_system {
        path.push(current_system);
        if let Some(&prev_system) = predecessors.get(&current_system) {
            current_system = prev_system;
        } else {
            break; // No predecessor found, path incomplete
        }
    }

    path.push(start_system); // Add start system at the end
    path.reverse(); // Reverse the path to get it from start to goal
    path
}
