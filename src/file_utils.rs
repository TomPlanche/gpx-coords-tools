/// # file_utils
/// This file contains functions for file manipulation.
///

use gpx::{read};

use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::{BufReader, Write};
use std::path::PathBuf;

use crate::gpx_utils::Coord;


const INVALID_FILENAME: &str = "InvalidFileName";

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
pub(crate) fn read_gpx_file(path: &PathBuf) -> Vec<Coord> {
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
pub(crate) fn read_file_name(path: &PathBuf) -> &str {
    // Use the file_name() method to get the file name
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            return file_name_str;
        }
    }
    // If the path does not contain a valid file name, return a default value or handle it as needed.
    INVALID_FILENAME
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
    file_destination: &PathBuf,
    file_coords_map: HashMap<&str, HashMap<&str, Vec<(usize, usize)>>>
) {
    // Create the file
    let mut file = File::create(file_destination).unwrap();

    // Write the HashMap to the file
    file.write_all(
        serde_json::to_string(&file_coords_map).unwrap().as_bytes()
    ).unwrap();
}
