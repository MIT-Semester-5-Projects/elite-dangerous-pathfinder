use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::coords::{euclidean, get_coords, get_neighbours};
use crate::system::{DijkState, StarSystem};

pub fn dijkstra(
    start_system: u64,
    goal_system: u64,
    jump_distance: f64,
    map_data: &[StarSystem],
) -> (HashMap<u64, f64>, Vec<u64>) {
    let mut epoch = 0;

    let mut priority_queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut costs = HashMap::new();
    let mut predecessors = HashMap::new(); // To track the predecessor of each system

    costs.insert(start_system, 0.0);
    priority_queue.push(DijkState {
        cost: 0.0,
        system_id: start_system,
    });

    while let Some(DijkState {
        cost: curr_cost,
        system_id: curr_node,
    }) = priority_queue.pop()
    {
        if visited.contains(&curr_node) {
            continue;
        }

        visited.insert(curr_node);

        // Debugging information
        print!("\x1B[2J\x1B[1;1H"); // Clear the screen and move cursor to (1,1)
        println!("Dijkstra's Pathfinding");
        println!("Start System: {}", start_system);
        println!("Goal System: {}", goal_system);
        println!("Current Epoch: {}", epoch);
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
                    priority_queue.push(DijkState {
                        cost: new_cost,
                        system_id: neighbour_id,
                    });
                }
            }
        }

        epoch += 1;
        // if epoch % 100 == 0 {
        //     println!("{:?}", costs);
        // }
    }

    let path = reconstruct_path(&predecessors, start_system, goal_system);

    (costs, path)
}

// Function to reconstruct the path from start to goal using the predecessor map
fn reconstruct_path(
    predecessors: &HashMap<u64, u64>,
    start_system: u64,
    goal_system: u64,
) -> Vec<u64> {
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
