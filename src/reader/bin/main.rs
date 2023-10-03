/// # reader main
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

#[path = "../../file_utils.rs"]
mod file_utils;

use file_utils::get_final_json_path;
use ansi_term::Colour::{Green, Red};

fn main() {
    // Check if the destination file exists
    if !get_final_json_path().exists() {
        println!("{}", Red.paint("The destination file does not exist"));
        println!("{}", Green.paint("Run the comparator first"));
        return;
    } else {
        println!("{}", Green.paint("The destination file exists"));
    }
}
