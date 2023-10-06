/// # reader.rs
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

#[path = "gpx_utils.rs"]
mod gpx_utils;

#[path = "file_utils.rs"]
mod file_utils;

#[path = "utils.rs"]
mod utils;

use std::collections::HashMap;
use std::env;
use file_utils::get_final_json_path;
use ansi_term::Colour::{Green, Red};
use serde::de::value::StringDeserializer;
use crate::file_utils::load_from_json;

fn main() {
    // Check if the destination file exists
    if !get_final_json_path().exists() {
        println!("{}", Red.paint("The destination file does not exist"));
        println!("{}", Green.paint("Run the comparator first"));
        return;
    } else {
        println!("{}", Green.paint("The destination file exists"));
    }

    let args: Vec<String> = env::args().collect();
    let mut file_names: Vec<String> = Vec::new();

    // Get the file names from the arguments
    for arg in args.iter().skip(1) {
        file_names.push(arg.to_string());
    }

    let file_coords_map: HashMap<String, HashMap<String, Vec<(usize, usize)>>> = match load_from_json(&get_final_json_path()) {
        Ok(map) => {
            map
        }
        Err(err) => {
            println!("{}", Red.paint("Error while loading the destination file"));
            println!("{}", err);
            return;
        }
    };

    println!("{:?}", file_coords_map);
}
