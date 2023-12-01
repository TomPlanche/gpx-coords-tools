///
/// # folder_to_json.rs
/// This file takes a folder as input and outputs a JSON file.
/// It will iterate through the folder and its subfolders to find all files and their
/// paths.
///
/// # # Author
/// Tom Planche <github.com/tomPlanche>

// Imports  ==============================================================================  Imports
#[path = "file_utils.rs"]
mod file_utils;

use json::JsonValue;

use std::{
    env::{
        current_dir,
        args,
    },
    io::Write,
    path::Path,
};
use std::path::PathBuf;

use crate::file_utils::{iterate_over_folder, Mode};
// Variables  =========================================================================== Variables


// Functions  =========================================================================== Functions
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
        None => "output/all_requires.json".to_string(),
    };

    if !Path::new(&folder_path).exists() {
        panic!("The folder {:?} does not exist", folder_path);
    }

    // final json is a simple array
    let final_json: JsonValue = json::array![];

    // write '[' to the file
    let mut file = std::fs::File::create(&file_destination).unwrap();
    file.write_all(b"[").unwrap();


    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    iterate_over_folder(
        final_json,
        Path::new(&folder_path),
        Mode::Files,
        Some(folder_path.to_string()),
        Some(&PathBuf::from(file_destination.clone())),
    );

    // write ']' to the file

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&file_destination)
        .unwrap();

    file.write_all(b"]").unwrap();

}

/*
 * End of file /folder_to_json.rs
 */
