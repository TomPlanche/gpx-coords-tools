/// # file_utils.rs
/// This file contains functions for file manipulation.
///
/// ## Author
/// * Tom Planche - <github.com/tomPlanche>

// IMPORTS ===================================================================================================  IMPORTS
mod utils;

#[path = "gpx_utils.rs"]
mod gpx_utils;


use gpx::{read};

use std::{
    collections::HashMap,
    fs::{
        File,
        read_dir
    },
    io::{
        BufReader,
        Read,
        Write
    },
    path::{
        Path,
        PathBuf
    },
};
use std::fs::OpenOptions;

use json::JsonValue;

use utils::{FileCoordsHM};
use gpx_utils::Coord;
// END IMPORTS ==========================================================================================   END IMPORTS

// VARIABLES ================================================================================================ VARIABLE
const PATH_PREFIX: &str = "../../assets";
// END VARIABLES ======================================================================================= END VARIABLES

// FUNCTIONS ================================================================================================ FUNCTIONS
///
/// # file_name_to_path_buf
/// Convert a file name to a PathBuf.
/// The file name is relative to the 'assets' directory.
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn get_final_json_path() -> PathBuf {
    // Construct the path to 'final.json'
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Get the project's root directory
    path.push("./output");
    path.push("final.json");

    path
}

///
/// # look_4_files
/// Look for GPX files in the 'assets' directory and return a vector of PathBuf
///
/// ## Returns
/// * `Vec<PathBuf>` - A vector of PathBuf.
#[allow(dead_code)]
pub fn look_4_files() -> Vec<PathBuf> {
    let paths = read_dir("./assets/").unwrap();

    paths
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() && path.extension().unwrap() == "gpx" {
                return Some(path);
            }

            None
        })
        .collect()
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
#[allow(dead_code)]
pub fn read_gpx_file(path: &PathBuf) -> Option<Vec<Coord>> {
    match read(BufReader::new(File::open(path).unwrap())) {
        Ok(gpx) => {
            let mut coords: Vec<Coord> = Vec::new();
            for track in gpx.tracks {
                for segment in track.segments {
                    for point in segment.points {
                        coords.push(Coord {
                            lat: point.point().y(),
                            lon: point.point().x(),
                        });
                    }
                }
            }

            Some(coords)
        }
        Err(error) => {
            println!("Error: : {}", error);
            None
        }
    }
}

/// # read_file_name
/// Read a file name from a PathBuf
///
/// ## Arguments
/// * `path(&PathBuf)` - The path to the file.
///
/// ## Returns
/// * `&str` - The file name
#[allow(dead_code)]
pub fn read_file_name(path: &PathBuf) -> Option<String> {
    // Use the file_name() method to get the file name
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            return Some(file_name_str.to_string());
        }
    }
    // If the path does not contain a valid file name, return a default value or handle it as needed.
    None
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
#[allow(dead_code)]
pub fn save_to_json(
    file_coords_map: HashMap<String, HashMap<String, Vec<(usize, usize)>>>
) -> bool{
    // Create the file
    let mut file = File::create(get_final_json_path()).unwrap();

    // Write the HashMap to the file
    match file.write_all(serde_json::to_string(&file_coords_map).unwrap().as_bytes()) {
        Ok(_) => {
            println!("Successfully saved to: {}", get_final_json_path().display());
            true
        }
        Err(error) => {
            println!("Error: {}", error);
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
#[allow(dead_code)]
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

///
/// # folder_contains_folder
/// Checks if a folder contains a folder.
///
/// ## Arguments
/// * `folder_path` - The path of the folder to check
///
/// ## Returns
/// * `bool` - True if the folder contains a folder, false otherwise
#[allow(dead_code)]
pub fn folder_contains_folder(folder_path: &Path) -> bool {
    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    for element in read_dir(&folder_path).unwrap() {
        if !element.is_ok() {
            // If the element is not ok, skip it
            continue;
        }

        let element = element.unwrap(); // Get the element
        let element_path = element.path(); // Get the path of the element

        if element_path.is_dir() {
            return true;
        }
    }

    false
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
#[allow(dead_code)]
pub enum Mode {
    Tiles,
    Files
}

///
/// # iterate_over_folder
/// Iterates over a folder and returns a json with the files and folders.
/// If the element is a file, add it to the final json.
/// If the element is a folder, add it to the final json and iterate over it.
///
/// ## Arguments
/// * `final_json` - The json to add the files and folders to.
/// * `folder_path` - The path of the folder to iterate over.
/// * `mode` - The mode to use.
/// * `Optional<&str>` - The part of the folder path to remove.
/// * `Optional<&PathBuf>` - The file to write to.
///
/// ## Returns
/// * `json::JsonValue` - The json with the files and folders.
#[allow(dead_code)]
pub fn iterate_over_folder(
    mut final_json: JsonValue,
    folder_path: &Path,
    mode: Mode,
    folder_part_to_remove: Option<String>,
    file_to_write_to: Option<&PathBuf>,
) -> JsonValue {
    // Check if the folder exists
    if !folder_path.exists() {
        panic!("The folder {:?} does not exist - ALED", folder_path);
    }

    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    for element in read_dir(&folder_path).unwrap() {
        if !element.is_ok() {
            // If the element is not ok, skip it
            continue;
        }

        let element = element.unwrap(); // Get the element
        let element_path = element.path(); // Get the path of the element
        let element_name = element_path.file_name().unwrap().to_str().unwrap(); // Get the name of the element

        // since at the root we only have folders, we can skip the check
        // for files (tiles.json)
        if element_path.is_dir() {

            match mode {
                Mode::Tiles => {
                    final_json[element_name] = if folder_contains_folder(&element_path) {
                        iterate_over_folder(json::object! {}, &element_path, mode, None, None)
                    } else {
                        create_file_list(&element_path)
                    };
                },
                Mode::Files => {

                    if folder_contains_folder(&element_path) {
                        iterate_over_folder(
                            final_json.clone(),
                            &element_path, mode,
                            folder_part_to_remove.clone(),
                            file_to_write_to.clone()
                        );
                    } else {
                        create_required_files(
                            &element_path,
                            folder_part_to_remove.clone().unwrap(),
                            file_to_write_to.clone()
                        );
                    }
                }
            }
        }
    }

    final_json // Return the final json
}

/// # create_required_files
/// Push the required files for the project in the array of the json.
///
/// ## Arguments
/// * `folder_path` - The path of the folder to iterate over.
/// * `folder_part_to_remove` - The part of the folder path to remove.
/// * `file_to_write_to` - The file to write to.
///
/// ## Returns
/// * `String` - The file name.
fn create_required_files(
    folder_path: &Path,
    folder_part_to_remove: String,
    file_to_write_to: Option<&PathBuf>,
) {
    // Check if the folder exists
    if !folder_path.exists() {
        panic!("The folder {:?} does not exist - ALED", folder_path);
    }


    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    for element in read_dir(&folder_path).unwrap() {
        if !element.is_ok() {
            // If the element is not ok, skip it
            continue;
        }

        let element = element.unwrap(); // Get the element
        let element_path = element.path(); // Get the path of the element

        if element_path.is_file() {
            // add the file to the json array,
            // the required is of the form:
            // require(../../assets/Chupaca/12/1191/2186.png)
            // let file_path = format!("require({}{
            // remove the folder_part_to_remove from the element_path
            let element_path = element_path.to_str().unwrap().replace(&folder_part_to_remove, "");

            let final_string = format!("require('{}{}'),", PATH_PREFIX, element_path);

            // write the string to the file
            let mut file_ref = OpenOptions::new().append(true).open(file_to_write_to.unwrap()).unwrap();

            if let Some(file_to_write_to) = file_to_write_to {
                match file_ref.write_all(final_string.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => panic!("Could not write to file: {}", e),
                }
            }
        }
    }

}

///
/// # create_file_list
/// Creates a json with the files into a list.
///
/// ## Arguments
/// * `folder_path` - The path of the folder to iterate over.
///
/// ## Returns
/// * `json::JsonValue` - The json with the files and folders.
pub fn create_file_list(folder_path: &Path) -> JsonValue {
    // Check if the folder exists
    if !folder_path.exists() {
        panic!("The folder {:?} does not exist - ALED", folder_path);
    }

    let mut final_json: JsonValue = json::array! [];

    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    for element in read_dir(&folder_path).unwrap() {
        if !element.is_ok() {
            // If the element is not ok, skip it
            continue;
        }

        let element = element.unwrap(); // Get the element
        let element_path = element.path(); // Get the path of the element
        let element_name = element_path.file_name().unwrap().to_str().unwrap(); // Get the name of the element

        if element_path.is_file() {
            final_json.push(element_name).expect("TODO: ALEDAOSSAZED");
        }
    }

    final_json // Return the final json
}
// END FUNCTIONS =======================================================================================  END FUNCTIONS

//
// * End of file file_utils.rs
//
