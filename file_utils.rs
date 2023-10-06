/// # file_utils.rs
/// This file contains functions for file manipulation.
///
/// ## Author
/// * Tom Planche - <github.com/tomPlanche>

// IMPORTS ===================================================================================================  IMPORTS
use gpx::{read};

use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

use crate::gpx_utils::Coord;
use crate::utils::{FileCoordsHM};
// END IMPORTS ==========================================================================================   END IMPORTS

// VARIABLES ================================================================================================ VARIABLE
// Constants
const INVALID_FILENAME: &str = "InvalidFileName";
// END VARIABLES ======================================================================================= END VARIABLES

// FUNCTIONS ================================================================================================ FUNCTIONS
///
/// # file_name_to_path_buf
/// Convert a file name to a PathBuf.
/// The file name is relative to the 'assets' directory.
pub fn file_name_to_path_buf(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Get the project's root directory
    path.push("./assets");
    path.push(file_name);

    path
}

///
/// # get_final_json_path
/// Get the path to the 'final.json' file.
///
/// ## Returns
/// * `PathBuf` - The path to the 'final.json' file.
pub fn get_final_json_path() -> PathBuf {
    // Construct the path to 'final.json'
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Get the project's root directory
    path.push("./destination");
    path.push("final.json");

    path
}

///
/// # look_4_files
/// Look for GPX files in the 'assets' directory and return a vector of PathBuf
///
/// ## Returns
/// * `Vec<PathBuf>` - A vector of PathBuf.
pub(crate) fn look_4_files() -> Vec<PathBuf> {
    let paths = read_dir("./assets/").unwrap();

    return paths
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if path.extension().unwrap() == "gpx" {
                    return Some(path);
                }
            }
            return None;
        })
        .collect();
}

///
/// # read_gpx_file
/// Read a GPX file and return its content as a string.
///
/// ## Arguments
/// * `path(&PathBuf)` - The path to the GPX file
///
/// ## Returns
/// * `Vector<Coord>` - A vector of coordinates.
pub fn read_gpx_file(path: &PathBuf) -> Vec<Coord> {
    return match read(BufReader::new(File::open(path).unwrap())) {
        Ok(gpx) => {
            let mut coords: Vec<Coord> = Vec::new();
            for track in gpx.tracks {
                for segment in track.segments {
                    for point in segment.points {
                        coords.push(Coord {
                            lat: point.point().x(),
                            lon: point.point().y(),
                        });
                    }
                }
            }

            coords
        }
        Err(error) => {
            println!("{}: {}", "Error: ", error);
            Vec::new()
        }
    };
}

/// # read_file_name
/// Read a file name from a PathBuf
///
/// ## Arguments
/// * `path(&PathBuf)` - The path to the file.
///
/// ## Returns
/// * `&str` - The file name
pub(crate) fn read_file_name(path: &PathBuf) -> String {
    // Use the file_name() method to get the file name
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            return file_name_str.to_string();
        }
    }
    // If the path does not contain a valid file name, return a default value or handle it as needed.
    INVALID_FILENAME.to_string()
}

///
/// # save_to_json
/// Save the HashMap<&str, HashMap<&str, Vec<(usize, usize)>>> to a JSON file.
///
/// ## Arguments
/// * `file_destination(&PathBuf)` - The path to the JSON file
/// * `file_coords_map(HashMap<&str, HashMap<&str, Vec<(usize, usize)>>>)` - The HashMap to save
///
/// ## Returns
/// * `bool` - True if the HashMap was saved, false otherwise
pub(crate) fn save_to_json(
    file_coords_map: HashMap<String, HashMap<String, Vec<(usize, usize)>>>
) -> bool{
    // Create the file
    let mut file = File::create(get_final_json_path()).unwrap();

    // Write the HashMap to the file
    match file.write_all(serde_json::to_string(&file_coords_map).unwrap().as_bytes()) {
        Ok(_) => {
            println!("{}: {}", "Successfully saved to", get_final_json_path().display());
            true
        }
        Err(error) => {
            println!("{}: {}", "Error: ", error);
            false
        }
    }
}

/// # load_from_json
/// Load a HashMap<&str, HashMap<&str, Vec<(usize, usize)>>> from a JSON file.
///
/// ## Arguments
/// * `file_path(&PathBuf)` - The path to the JSON file
///
/// ## Returns
/// * `Result<HashMap<&str, HashMap<&str, Vec<(usize, usize)>>>` - A Result containing the loaded HashMap or an error
pub fn load_from_json(
    file_path: &PathBuf,
) -> FileCoordsHM {
    // Open the file
    let mut file = File::open(file_path).expect("The file could not be opened");

    // Read the contents of the file into a string
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).expect("The file could not be read");

    // Parse the JSON string into a HashMap
    let result: FileCoordsHM =
        serde_json::from_str(&json_string).expect("The JSON string could not be parsed");


    result
}

// END FUNCTIONS =======================================================================================  END FUNCTIONS

//
// * End of file file_utils.rs
//
