use serde_json::Value;
use std::{fs::File, io::BufReader};

mod coords;
mod dijkstra;
mod parse;

use dijkstra::naive_dijkstra;
use parse::parse_map_data;

fn main() {
    // Load map data from JSON
    let file = File::open("../map-data/systems_1week.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let map_data_json: Value = serde_json::from_reader(reader).expect("Could not parse JSON");

    // Convert JSON data into a Vec<System>
    let map_data = parse_map_data(&map_data_json);

    // Define start and goal system IDs
    let start_system = 10477373803;
    let goal_system = 6681123623626;

    // Run the naive_dijkstra algorithm
    let (_results, path) = naive_dijkstra(start_system, goal_system, &map_data);

    if !path.is_empty() {
        println!("Path found: {:?}", path);
    } else {
        println!("No path found.");
    }
}
