/// # Main
/// The goal of this is to find the common coordinates between GPX files.
///
/// The algorithm is as follows:
/// 1. Read the GPX files
/// 2. For each unique pair of GPX files ((file_1, file_2) = (file_2, file_1)) so do not repeat:
///    1. For each coordinate in file_1:
///      1. For each coordinate in file_2:
///       1. If the coordinates are the same:
///        1. Add the coordinates to the common coordinates map
/// 3. Print the common coordinates map
///

mod gpx_utils;
mod file_utils;
mod utils;

use crate::gpx_utils::{calc_distance, Coord};
use crate::file_utils::{look_4_files, read_file_name, read_gpx_file};
use crate::utils::get_unique_pairs;

use std::path::PathBuf;
use std::collections::{HashMap};

fn main() {
    let file_destination: PathBuf = PathBuf::from("./destination/final.json");

    // Map of file names to Vec<Coords>
    let mut gpx_coords_map: HashMap<
        &str,
        Vec<Coord>
    > = HashMap::new();

    // Final map containing for each pair of files the common coordinates
    let mut file_coords_map: HashMap<
        &str,
        HashMap<
            &str,
            Vec<(usize, usize)>
        >
    > = HashMap::new();

    // Get the GPX files
    let gpx_files: Vec<PathBuf> = look_4_files();

    // Init the gpx_coords_map with the actual coordinates
    for file in &gpx_files {
        gpx_coords_map.insert(
            read_file_name(file),
            read_gpx_file(file)
        );
    }

    // Get the pair of files in order to compare them and avoid repeating
    let pairs: Vec<(&PathBuf, &PathBuf)> = get_unique_pairs(&gpx_files);
    for (file_1, file_2) in pairs {
        println!("Comparing {:?} and {:?}", file_1, file_2);

        // Get the file names
        let file_1_name: &str = read_file_name(&file_1);
        let file_2_name: &str = read_file_name(&file_2);

        // Get the coordinates
        let file_1_coords: &Vec<Coord> = gpx_coords_map.get(file_1_name).unwrap();
        let file_2_coords: &Vec<Coord> = gpx_coords_map.get(file_2_name).unwrap();

        // Init the file_1_coords_map
        file_coords_map
            .entry(file_1_name)
            .or_insert_with(HashMap::new);

        // Compare the coordinates
        for (index_1, coord_1) in file_1_coords.iter().enumerate() {
            for (index_2, coord_2) in file_2_coords.iter().enumerate() {
                if calc_distance(*coord_1, *coord_2, Some(true)) < 10.0 {
                    // Add the coordinates to the file_coords_map
                    // since they are only unique pairs, no need to add to file_2

                    // Add the coordinates to the file_1_coords_map
                    file_coords_map
                        .get_mut(file_1_name)
                        .unwrap()
                        .entry(file_2_name)
                        .or_insert_with(Vec::new)
                        .push((index_1, index_2));
                }
            }
        }

        // If there are no common coordinates, remove the file_1_name from the file_coords_map
        if file_coords_map.get(file_1_name).unwrap().len() < 1 {
            file_coords_map.remove(file_1_name);
        }
    }

    // Save the file_coords_map to a file
    file_utils::save_to_json(&file_destination, file_coords_map);
}
