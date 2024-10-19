use crate::{coords::Coord, system::StarSystem};
use serde_json::Value;

pub fn parse_map_data(map_data_json: &Value) -> Vec<StarSystem> {
    let mut map_data = Vec::new();

    if let Some(systems) = map_data_json.as_array() {
        for system in systems {
            if let (Some(id64), Some(coords), Some(star_class)) = (
                system.get("id64").and_then(|id| id.as_u64()),
                system.get("coords"),
                system.get("mainStar").and_then(|star| star.as_str()), // Extract the star class
            ) {
                let x = coords.get("x").and_then(|x| x.as_f64()).unwrap_or(0.0);
                let y = coords.get("y").and_then(|y| y.as_f64()).unwrap_or(0.0);
                let z = coords.get("z").and_then(|z| z.as_f64()).unwrap_or(0.0);
                map_data.push(StarSystem {
                    id64,
                    coords: Coord { x, y, z },
                    star_class: star_class.to_string(), // Store the star class as String
                });
            }
        }
    }

    map_data
}
