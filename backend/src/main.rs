#[allow(dead_code)]
#[allow(unused_imports)]
use astar::a_star;
use dijkstra::dijkstra;
use parse::parse_map_data;

mod astar;
mod coords;
mod dijkstra;
mod parse;
mod system;
mod tests {
    mod search_test;
}
use axum::http::Method;

use axum::{extract::Json, routing::post, Router};

use std::{fs::File, io::BufReader, time::Instant};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(unused_imports)]
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize, Serialize, Debug)]
struct PathDetails {
    source: u64,
    destination: u64,
    via_systems: Vec<usize>,
    jump_distance: f64,
    weight: f64,
    algorithm: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Path {
    jumps: usize,
    path: Vec<(u64, f64, i32)>,
    distance: f64,
}

#[tokio::main]
async fn main() {
    // Configure CORS to allow all origins and required methods/headers
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin (for development)
        .allow_methods(vec![Method::GET, Method::POST]) // Allow GET and POST methods
        .allow_headers(Any); // Allow any headers

    // Define the routes
    let app = Router::new()
        .route("/api/path-details", post(find_path)) // POST route to handle form submissions
        .layer(cors); // Apply CORS middleware to the whole app

    // Bind and run the app on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Listening on 3000");
}

// Handler function to process the path details submitted from the frontend

async fn find_path(Json(payload): Json<PathDetails>) -> Json<Path> {
    // Log the received data for debugging
    println!("Received PathDetails: {:?}", payload);

    let file = File::open("map-data/systems_1week.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let map_data_json: Value = serde_json::from_reader(reader).expect("Could not parse JSON");

    // Convert JSON data into a Vec<System>
    let map_data = parse_map_data(&map_data_json);
    //

    let source = payload.source;
    let destination = payload.destination;
    // let via_systems = &payload.via_systems;
    let jump_distance = payload.jump_distance;
    let ship_weight = payload.weight;
    let algorithm = payload.algorithm;

    // let (results, path) = a_star(source, destination, jump_distance, ship_weight, &map_data);

    let (results, path);

    if algorithm == "Dijkstra" {
        let now = Instant::now();
        // Assign values to results and path for Dijkstra
        let (dijkstra_results, dijkstra_path) =
            dijkstra(source, destination, jump_distance, &map_data);
        let elapsed = now.elapsed();
        println!("Elapsed (Dijkstra) {:.2?}", elapsed);
        results = dijkstra_results;
        path = dijkstra_path;
    } else {
        let now = Instant::now();
        // Assign values to results and path for A*
        let (a_star_results, a_star_path) =
            a_star(source, destination, jump_distance, ship_weight, &map_data);
        let elapsed = now.elapsed();
        println!("Elapsed (A-Star) {:.2?}", elapsed);
        results = a_star_results;
        path = a_star_path;
    }

    if !path.is_empty() {
        println!("Path found: {:?}", path);
    } else {
        println!("No path found.");
    }

    let distance = if let Some(x) = results.get(&destination) {
        x
    } else {
        &0.00
        // let direct_distance = euclidean();
        // if direct_distance > jump_distance {
        //     0
        // } else {
        //     direct_distance
        // }
    };

    let final_path: Vec<(u64, f64, i32)> = path
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| {
            // Get the corresponding f64 value from the HashMap
            if let Some(&result) = results.get(&value) {
                Some((value, result, index as i32)) // Create the tuple
            } else {
                None // In case the value is not found in results
            }
        })
        .collect();

    // Create a response based on the received data
    let response = Path {
        jumps: path.len() - 1,
        path: final_path,
        // dhstance: *results.get(&destination).unwrap(),
        distance: *distance,
    };

    Json(response)
}

// fn main() {
//     // Load map data from JSON
//
//     let file = File::open("map-data/systems_1week.json").expect("Could not open file");
//     let reader = BufReader::new(file);
//     let map_data_json: Value = serde_json::from_reader(reader).expect("Could not parse JSON");
//
//     let map_data = parse_map_data(&map_data_json);
//
//     let start_system = 10477373803;
//     let goal_system = 6681123623626;
//
//     // let (results, path) = dijkstra(start_system, goal_system, 60.0, &map_data);
//     let (results, path) = a_star(start_system, goal_system, 60.0, 30000.0, &map_data);
//
//     if !path.is_empty() {
//         println!("Path found: {:?}", path);
//     } else {
//         println!("No path found.");
//     }
//     println!("{:?}", results)
// }
