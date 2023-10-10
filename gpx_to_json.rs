/// # gpx_to_json.rs
///
///
/// /// * Tom Planche - <github.com/tomPlanche>

// IMPORTS ===================================================================================================  IMPORTS
#[path = "gpx_utils.rs"]
mod gpx_utils;

#[path = "file_utils.rs"]
mod file_utils;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use file_utils::{file_name_to_path_buf, read_gpx_file};
use crate::file_utils::look_4_files;
use crate::gpx_utils::Coord;

// END IMPORTS ==========================================================================================   END IMPORTS

// VARIABLES ================================================================================================ VARIABLE
// Type(s)

// Other(s)
// END VARIABLES ======================================================================================= END VARIABLES

// CODE ========================================================================================================= CODE
fn gpx_to_json(file_name: String, file_destination: String) -> bool {
    let path_buff_from_file_1: PathBuf = file_name_to_path_buf(&file_name);

    let coords: Vec<Coord> = match read_gpx_file(&path_buff_from_file_1) {
        Some(coords) => coords,
        None => panic!("Could not read the file {:?}", file_name),
    };

    let mut file = File::create(&file_destination).unwrap();

    match file.write_all(serde_json::to_string(&coords).unwrap().as_bytes()) {
        Ok(_) => {
            println!("Successfully saved to: {}", file_destination);
            true
        }
        Err(error) => {
            println!("Error: {}", error);
            false
        }
    }
}

fn main() {
    let gpx_files: Vec<PathBuf> = look_4_files();

    for file in gpx_files {
        let file_name: String = match file.file_name() {
            Some(file_name) => file_name.to_str().unwrap().to_string(),
            None => panic!("Could not read the file name of {:?}", file),
        };

        let file_destination: String = format!("./destination/{}.json", file_name);

        gpx_to_json(file_name, file_destination);
    }
}
// END CODE =======================================================================================  END COMPONENT

//
// * End of file /gpx_to_json.rs
//
