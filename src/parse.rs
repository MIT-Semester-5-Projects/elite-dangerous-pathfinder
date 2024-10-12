use crate::coords::{Coord, System};
use serde_json::Value;

pub fn parse_map_data(map_data_json: &Value) -> Vec<System> {
    let mut map_data = Vec::new();

    if let Some(systems) = map_data_json.as_array() {
        for system in systems {
            if let (Some(id64), Some(coords)) = (
                system.get("id64").and_then(|id| id.as_i64()),
                system.get("coords"),
            ) {
                let x = coords.get("x").and_then(|x| x.as_f64()).unwrap_or(0.0);
                let y = coords.get("y").and_then(|y| y.as_f64()).unwrap_or(0.0);
                let z = coords.get("z").and_then(|z| z.as_f64()).unwrap_or(0.0);
                map_data.push(System {
                    id64,
                    coords: Coord { x, y, z },
                });
            }
        }
    }

    map_data
}
