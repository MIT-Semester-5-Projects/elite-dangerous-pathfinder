use csv::Writer;
use serde_json::Value;
use std::error::Error;
use std::time;
use std::{fs::File, io::BufReader};

mod parse;

use crate::parse::parse_map_data;

fn bin_search(system_id: i64, map_data: &[System]) -> Coord {
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
}

fn lin_search(system_id: i64, map_data: &[System]) -> Coord {
    for (id, coord) in map_data.enumerate() {
        if id == system_id {
            return coord;
        }
    }
}
fn main() {
    // Loading JSON File
    let file = File::open("../../map-data/systems_1month.json").expect("File Not Found");
    let reader = BufReader::new(file);
    let map_data_json: Value = serde_json::from_reader(reader).expect("Could Not Parse JSON");
    let map_data = parse_map_data(map_data_json);

    let mut lin_writer = Writer::from_path("lin_search.csv");
    let mut bin_writer = Writer::from_path("bin_search.csv");

    for test_no in 1..100 {
        //splice map_data in increments of 20,000
        //input_size = map_data.len()
        //time_start
        //call bin_search
        // time_end
        //time_start
        //call lin_search
        //time end
        // write(time_taken, input size) to both csv files
        //end
    }
}
