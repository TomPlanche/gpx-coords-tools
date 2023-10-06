/// # comparator.rs
/// This module will compare the GPX files in the 'gpx_files' folder and will output a JSON file
/// with a map of the common coordinates between the files.
///
/// The final map will look like this:
/// ```json
/// {
///   "file_1": {
///     "file_2": [(index_file_1, index_file_2), ...],
///     "file_3": [(index_file_1, index_file_3), ...],
///     ...
///   },
///   "file_2": {
///     ...
///   },
///   ...
/// }
/// ```
///
/// ## Author
/// * Tom Planche - <github.com/tomPlanche>

// IMPORTS ===================================================================================================  IMPORTS
#[path = "gpx_utils.rs"]
mod gpx_utils;

#[path = "file_utils.rs"]
mod file_utils;

#[path = "utils.rs"]
mod utils;

use crate::gpx_utils::{calc_distance, Coord};
use crate::file_utils::{look_4_files, read_file_name, read_gpx_file, save_to_json};
use crate::utils::{FileCoordsHM, get_unique_pairs};

use std::path::PathBuf;
use std::collections::{HashMap};
// END IMPORTS ==========================================================================================   END IMPORTS


// FUNCTIONS ================================================================================================ FUNCTIONS
fn main() {
    // Map of file names to Vec<Coords>
    let mut gpx_coords_map: HashMap<
        String,
        Vec<Coord>
    > = HashMap::new();

    // Final map containing for each pair of files the common coordinates
    let mut file_coords_map: FileCoordsHM = HashMap::new();

    // Get the GPX files
    let gpx_files: Vec<PathBuf> = look_4_files();

    // Init the gpx_coords_map with the actual coordinates
    for file in &gpx_files {
        let file_name: String = match read_file_name(file) {
            Some(file_name) => file_name,
            None => panic!("Could not read the file name of {:?}", file),
        };

        let coords: Vec<Coord> = match read_gpx_file(file) {
            Some(coords) => coords,
            None => panic!("Could not read the file {:?}", file),
        };

        gpx_coords_map.insert(
            file_name,
            coords
        );
    }

    // Get the pair of files in order to compare them and avoid repeating
    let pairs: Vec<(&PathBuf, &PathBuf)> = get_unique_pairs(&gpx_files);
    for (file_1, file_2) in pairs {
        println!("Comparing {:?} and {:?}", file_1, file_2);

        // Get the file names
        let file_1_name: String = match read_file_name(file_1) {
            Some(file_name) => file_name,
            None => panic!("Could not read the file name of {:?}", file_1),
        };
        let file_2_name: String = match read_file_name(file_2) {
            Some(file_name) => file_name,
            None => panic!("Could not read the file name of {:?}", file_2),
        };

        // Get the coordinates
        let file_1_coords: &Vec<Coord> = gpx_coords_map.get(&*file_1_name).unwrap();
        let file_2_coords: &Vec<Coord> = gpx_coords_map.get(&*file_2_name).unwrap();

        // Init the file_1_coords_map
        file_coords_map
            .entry(file_1_name.clone())
            .or_default();

        // Compare the coordinates
        for (index_1, coord_1) in file_1_coords.iter().enumerate() {
            for (index_2, coord_2) in file_2_coords.iter().enumerate() {
                if calc_distance(*coord_1, *coord_2, Some(true)) < 10.0 {
                    // Add the coordinates to the file_coords_map
                    // since they are only unique pairs, no need to add to file_2

                    // Add the coordinates to the file_1_coords_map
                    file_coords_map
                        .get_mut(file_1_name.clone().as_str())
                        .unwrap()
                        .entry(file_2_name.clone())
                        .or_default()
                        .push((index_1, index_2));
                }
            }
        }

        // If there are no common coordinates, remove the file_1_name from the file_coords_map
        if file_coords_map.get(&*file_1_name).unwrap().is_empty() {
            file_coords_map.remove(&*file_1_name);
        }
    }

    // Save the file_coords_map to a file
    save_to_json(file_coords_map);
}

// END FUNCTIONS =======================================================================================  END FUNCTIONS

//
// * End of file file_utils.rs
//
