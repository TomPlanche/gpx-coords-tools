///
/// # tiles_to_json.rs
/// This file will be a binary, it will take a folder as an argument and will output a json file.
///
/// Tom Planche <github.com/tomPlanche>

// Imports  ==============================================================================  Imports
use json::JsonValue;

use std::{
    env::{
        current_dir,
        args,
    },
    fs::{
        self,
        File
    },
    io::Write,
    path::Path,
};
// Variables  =========================================================================== Variables

// Functions  =========================================================================== Functions
///
/// # folder_contains_folder
/// Checks if a folder contains a folder.
///
/// ## Arguments
/// * `folder_path` - The path of the folder to check
///
/// ## Returns
/// * `bool` - True if the folder contains a folder, false otherwise
fn folder_contains_folder(folder_path: &Path) -> bool {
    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    for element in fs::read_dir(&folder_path).unwrap() {
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

///
/// # create_file_list
/// Creates a json with the files into a list.
///
/// ## Arguments
/// * `folder_path` - The path of the folder to iterate over.
///
/// ## Returns
/// * `json::JsonValue` - The json with the files and folders.
fn create_file_list(folder_path: &Path) -> JsonValue {
    // Check if the folder exists
    if !folder_path.exists() {
        panic!("The folder {:?} does not exist - ALED", folder_path);
    }

    let mut final_json: JsonValue = json::array! [];

    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    for element in fs::read_dir(&folder_path).unwrap() {
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

///
/// # iterate_over_folder
/// Iterates over a folder and returns a json with the files and folders.
/// If the element is a file, add it to the final json.
/// If the element is a folder, add it to the final json and iterate over it.
///
/// ## Arguments
/// * `final_json` - The json to add the files and folders to.
/// * `folder_path` - The path of the folder to iterate over.
///
/// ## Returns
/// * `json::JsonValue` - The json with the files and folders.
fn iterate_over_folder(mut final_json: JsonValue, folder_path: &Path) -> JsonValue {
    // Check if the folder exists
    if !folder_path.exists() {
        panic!("The folder {:?} does not exist - ALED", folder_path);
    }

    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    for element in fs::read_dir(&folder_path).unwrap() {
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
            final_json[element_name] = if folder_contains_folder(&element_path) {
                iterate_over_folder(json::object! {}, &element_path)
            } else {
                create_file_list(&element_path)
            };
        }
    }

    final_json // Return the final json
}


fn main() {
    // Folder caller - the folder from which the program was called
    let caller = current_dir().unwrap();

    // Folder to read from
    let folder_path = match args().nth(1) {
        Some(folder_path) => folder_path,
        None => panic!("Please provide a folder path"),
    };

    let file_destination = match args().nth(2) {
        Some(file_destination) => {
            if !file_destination.ends_with(".json") {
                panic!("The file destination must end with .json");
            }

            file_destination
        },
        None => "output/tiles_struct.json".to_string(),
    };

    if !Path::new(&folder_path).exists() {
        panic!("The folder {:?} does not exist", folder_path);
    }

    let final_json: JsonValue = json::object! {};

    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    let final_json = iterate_over_folder(final_json, Path::new(&folder_path));

    // create/recreate the output file
    let mut file = File::create(format!("{}/{}", caller.display(), file_destination)).unwrap();

    // write the final json to the file
    file.write_all(final_json.dump().as_bytes()).unwrap();


}

/*
 * End of file /tiles_to_json.rs
 */
