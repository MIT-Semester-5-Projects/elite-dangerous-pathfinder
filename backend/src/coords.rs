#[derive(Clone, Copy, PartialEq)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy)]
pub struct System {
    pub id64: i64,
    pub coords: Coord,
}

pub fn get_coords(system_id: i64, map_data: &[System]) -> Coord {
    let mut low = 0;
    let mut high = map_data.len() as isize - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if map_data[mid as usize].id64 == system_id {
            return map_data[mid as usize].coords;
        } else if map_data[mid as usize].id64 < system_id {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    Coord {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

pub fn euclidean(point_a: Coord, point_b: Coord) -> f64 {
    ((point_a.x - point_b.x).powi(2)
        + (point_a.y - point_b.y).powi(2)
        + (point_a.z - point_b.z).powi(2))
    .sqrt()
}

pub fn get_neighbours(
    curr_system: i64,
    jump_distance: f64,
    map_data: &[System],
) -> Vec<(i64, Coord)> {
    let mut neighbours = Vec::new();
    let curr_coords = get_coords(curr_system, map_data);

    for node in map_data {
        let distance = euclidean(curr_coords, node.coords);
        if distance <= jump_distance {
            neighbours.push((node.id64, node.coords));
        }
    }

    neighbours
}
