
/// # find_closest_points.rs
/// This file contains functions for file manipulation.
///
/// ## Author
/// Tom Planche - <github.com/tomPlanche>

// IMPORTS ===================================================================================================  IMPORTS
#[path = "gpx_utils.rs"]
mod gpx_utils;

#[path = "file_utils.rs"]
mod file_utils;

use std::path::PathBuf;
use crate::file_utils::{file_name_to_path_buf, read_gpx_file};
use crate::gpx_utils::{calc_distance, Coord};
// END IMPORTS ==========================================================================================   END IMPORTS

// VARIABLES ================================================================================================ VARIABLE
// Type(s)

// Other(s)
// END VARIABLES ======================================================================================= END VARIABLES

// CODE ========================================================================================================= CODE
///
/// # find_closests_points
/// Find the n closest points to a given point in a given file.
///
/// ## Arguments
/// * `file_name` - The name of the file to read from.
/// * `point` - The point to compare to.
/// * `nb_points` - The number of points to return.
///
/// ## Returns
/// * `Vec<Coord>` - A vector of the n closest points.
fn find_closests_points(file_name: &str, point: Coord, nb_points: usize) -> Vec<Coord> {
    let path_buff_from_file_1: PathBuf = file_name_to_path_buf(file_name);

    let coords: Vec<Coord> = match read_gpx_file(&path_buff_from_file_1) {
        Some(coords) => coords,
        None => panic!("Could not read the file {:?}", file_name),
    };

    let mut indexes_distance: Vec<(usize, f64)> = coords
        .iter()
        .enumerate()
        .map(|(i, coord)| (i, calc_distance(point, *coord, Some(true))))
        .collect();

    indexes_distance
        .sort_by(|(_, dist_1), (_, dist_2)| dist_1.partial_cmp(dist_2).unwrap());

    if indexes_distance[0].1 == 0.0 {
        indexes_distance.remove(0);
    }

    indexes_distance
        .iter()
        .take(nb_points)
        .map(|(i, _)| coords[*i])
        .collect()
}

fn main() {
    println!("{:?}", find_closests_points("puertoviejofenars.gpx", Coord{lat: 42.6782078, lon: 0.0856054}, 2));
    todo!("Determinate the binary arguments and implement the function");
}
// END CODE =======================================================================================  END COMPONENT

//
// * End of file /find_closest_points.rs
//
