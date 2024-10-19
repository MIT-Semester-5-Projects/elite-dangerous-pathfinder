use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{
    coords::{euclidean, get_coords, get_neighbours, Coord},
    system::{AStarState, StarSystem},
};

fn heuristic(system: &StarSystem, goal_coords: Coord, ship_mass: f64) -> f64 {
    let distance_to_goal = euclidean(system.coords, goal_coords);

    // Star classes for refueling (KGB FOAM)
    let refuelable_star_classes = ["K", "G", "B", "F", "O", "A", "M"];

    // Assuming we have star class information stored in System struct
    let is_refuelable = refuelable_star_classes.contains(&system.star_class.as_str());

    let refuel_bonus = if is_refuelable { 0.75 } else { 1.0 }; // Refuelable systems are cheaper
    distance_to_goal * ship_mass * refuel_bonus
}

fn fuel_cost(distance: f64, ship_mass: f64) -> f64 {
    ship_mass * distance
}

pub fn a_star(
    start_system: u64,
    goal_system: u64,
    jump_distance: f64,
    ship_mass: f64,
    map_data: &[StarSystem],
) -> (HashMap<u64, f64>, Vec<u64>) {
    let mut epoch = 0;

    let mut priority_queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut fuel_costs = HashMap::new(); // Stores fuel costs
    let mut distance_costs = HashMap::new(); // Stores actual distances
    let mut predecessors = HashMap::new(); // To track the predecessor of each system

    let goal_coords = get_coords(goal_system, map_data);

    // Initialize start system
    fuel_costs.insert(start_system, 0.0);
    distance_costs.insert(start_system, 0.0);
    priority_queue.push(AStarState {
        cost: 0.0,      // g(n) (fuel cost)
        heuristic: 0.0, // h(n)
        system_id: start_system,
        ship_mass,
    });

    while let Some(AStarState {
        cost: curr_fuel_cost,
        system_id: curr_node,
        ..
    }) = priority_queue.pop()
    {
        if visited.contains(&curr_node) {
            continue;
        }

        visited.insert(curr_node);

        // Debugging information
        print!("\x1B[2J\x1B[1;1H"); // Clear the screen and move cursor to (1,1)
        println!("A-Star Pathfinding");
        println!("Start System: {}", start_system);
        println!("Goal System: {}", goal_system);
        println!("Current Epoch: {}", epoch);
        println!("Systems Visited: {}", visited.len());
        println!("Queued Systems: {}", priority_queue.len());
        println!("Fuel Cost: {}", fuel_costs[&curr_node]);
        println!("Distance: {}", distance_costs[&curr_node]);

        if curr_node == goal_system {
            break;
        }

        let neighbours = get_neighbours(curr_node, jump_distance, map_data);
        for (neighbour_id, neighbour_coords) in neighbours {
            if !visited.contains(&neighbour_id) {
                let distance = euclidean(get_coords(curr_node, map_data), neighbour_coords);
                let new_fuel_cost = curr_fuel_cost + fuel_cost(distance, ship_mass);
                let new_distance_cost = distance_costs[&curr_node] + distance; // Add actual distance

                if !fuel_costs.contains_key(&neighbour_id)
                    || new_fuel_cost < fuel_costs[&neighbour_id]
                {
                    fuel_costs.insert(neighbour_id, new_fuel_cost);
                    distance_costs.insert(neighbour_id, new_distance_cost);
                    predecessors.insert(neighbour_id, curr_node); // Update predecessor

                    let t = find_system_by_id(neighbour_id, map_data);
                    if let Some(x) = t {
                        let h = heuristic(x, goal_coords, ship_mass);
                        priority_queue.push(AStarState {
                            cost: new_fuel_cost,
                            system_id: neighbour_id,
                            heuristic: new_fuel_cost + h, // f(n) = g(n) + h(n)
                            ship_mass,
                        });
                    }
                }
            }
        }

        epoch += 1;
        if epoch % 100 == 0 {
            println!("{:?}", fuel_costs);
        }
    }

    let path = reconstruct_path(&predecessors, start_system, goal_system);

    (distance_costs, path) // Return actual distances instead of fuel costs
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

fn find_system_by_id(system_id: u64, map_data: &[StarSystem]) -> Option<&StarSystem> {
    map_data.iter().find(|&system| system.id64 == system_id)
}
