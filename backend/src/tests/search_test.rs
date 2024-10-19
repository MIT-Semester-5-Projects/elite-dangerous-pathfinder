#[cfg(test)]
mod tests {
    use super::*;
    use crate::coords::{Coord, System}; // Make sure these modules exist and are imported correctly
    use crate::parse::parse_map_data; // Ensure you have the right import
    use csv::Writer;
    use serde_json::Value;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::time::Instant;

    // Load your map data from the JSON file for testing
    fn load_map_data() -> Result<Vec<System>, Box<dyn Error>> {
        let file = File::open("../../map-data/systems_1month.json")?;
        let reader = BufReader::new(file);
        let map_data_json: Value = serde_json::from_reader(reader)?;
        let map_data = parse_map_data(map_data_json);
        Ok(map_data)
    }

    #[test]
    fn test_search_performance() {
        let map_data = load_map_data().expect("Failed to load map data");

        // Prepare CSV writers
        let mut lin_writer =
            Writer::from_path("lin_search_test.csv").expect("Failed to create CSV writer");
        let mut bin_writer =
            Writer::from_path("bin_search_test.csv").expect("Failed to create CSV writer");

        // Write CSV headers
        lin_writer
            .write_record(&["test_no", "input_size", "time_taken_micro"])
            .expect("Failed to write header");
        bin_writer
            .write_record(&["test_no", "input_size", "time_taken_micro"])
            .expect("Failed to write header");

        let total_iterations = 50; // Total number of iterations
        let total_size = map_data.len(); // Total size of map_data

        for test_no in 0..total_iterations {
            // Incrementally increase input size; the last iteration uses the entire length
            let input_size = (total_size * (test_no + 1)) / total_iterations;

            // Slice the sample data
            let sample_data = &map_data[..input_size];

            // Generate a random system_id to search for (you can customize this part)
            let system_id = sample_data[input_size / 2].id64; // using middle ID for testing

            // Measure binary search time
            let bin_start = Instant::now();
            let _ = bin_search(system_id, sample_data);
            let bin_duration = bin_start.elapsed().as_micros();

            // Measure linear search time
            let lin_start = Instant::now();
            let _ = lin_search(system_id, sample_data);
            let lin_duration = lin_start.elapsed().as_micros();

            // Write to CSV files
            bin_writer
                .write_record(&[
                    test_no.to_string(),
                    input_size.to_string(),
                    bin_duration.to_string(),
                ])
                .expect("Failed to write binary search record");

            lin_writer
                .write_record(&[
                    test_no.to_string(),
                    input_size.to_string(),
                    lin_duration.to_string(),
                ])
                .expect("Failed to write linear search record");
        }

        lin_writer
            .flush()
            .expect("Failed to flush linear search CSV");
        bin_writer
            .flush()
            .expect("Failed to flush binary search CSV");
    }

    #[test]
    fn test_bin_search_found() {
        let map_data = load_map_data().expect("Failed to load map data");
        let coord = bin_search(2, &map_data);
        assert!(coord.is_some());
        assert_eq!(
            coord.unwrap(),
            Coord {
                x: 4.0,
                y: 5.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn test_bin_search_not_found() {
        let map_data = load_map_data().expect("Failed to load map data");
        let coord = bin_search(99, &map_data); // ID that doesn't exist
        assert!(coord.is_none());
    }

    #[test]
    fn test_lin_search_found() {
        let map_data = load_map_data().expect("Failed to load map data");
        let coord = lin_search(3, &map_data);
        assert!(coord.is_some());
        assert_eq!(
            coord.unwrap(),
            Coord {
                x: 7.0,
                y: 8.0,
                z: 9.0
            }
        );
    }

    #[test]
    fn test_lin_search_not_found() {
        let map_data = load_map_data().expect("Failed to load map data");
        let coord = lin_search(99, &map_data); // ID that doesn't exist
        assert!(coord.is_none());
    }
}
