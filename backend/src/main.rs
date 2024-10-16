// mod astar;
mod coords;
mod dijkstra;
mod parse;
mod state;

// use astar::astar;
use dijkstra::dijkstra;
use parse::parse_map_data;
use std::{fs::File, io::BufReader};

use serde_json::Value;

use axum::http::Method;
use axum::{extract::Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer}; // Use tower-http's CorsLayer

#[derive(Deserialize, Serialize, Debug)]
struct PathDetails {
    source: u64,
    destination: u64,
    via_systems: Vec<usize>,
    jump_distance: f64,
    efficiency: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Path {
    jumps: usize,
    path: Vec<u64>,
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

    let file = File::open("../map-data/systems_1week.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let map_data_json: Value = serde_json::from_reader(reader).expect("Could not parse JSON");

    // Convert JSON data into a Vec<System>
    let map_data = parse_map_data(&map_data_json);
    //

    let source = payload.source;
    let destination = payload.destination;
    // let via_systems = &payload.via_systems;
    let jump_distance = payload.jump_distance;
    // let efficiency = payload.efficiency;

    let (_results, path) = dijkstra(source, destination, jump_distance, &map_data);

    if !path.is_empty() {
        println!("Path found: {:?}", path);
    } else {
        println!("No path found.");
    }

    // Create a response based on the received data
    let response = Path {
        jumps: path.len(),
        path,
    };

    Json(response)
}

// fn main() {
//     // Load map data from JSON
//     let file = File::open("../map-data/systems_1week.json").expect("Could not open file");
//     let reader = BufReader::new(file);
//     let map_data_json: Value = serde_json::from_reader(reader).expect("Could not parse JSON");
//
//     // Convert JSON data into a Vec<System>
//     let map_data = parse_map_data(&map_data_json);
//
//     // Define start and goal system IDs
//     let start_system = 10477373803;
//     let goal_system = 6681123623626;
//
//     // Run the naive_dijkstra algorithm
//     let (_results, path) = dijkstra(start_system, goal_system, &map_data);
//
//     if !path.is_empty() {
//         println!("Path found: {:?}", path);
//     } else {
//         println!("No path found.");
//     }
// }
