/// # calc_distance_from_json_point.rs
/// This file will be a binary.
/// It'll recieve a json for two Coordd and calculate the distance between them.
///
/// ## Author
/// * Tom Planche - <github.com/tomPlanche>
///
/// ## Example
/// >> cargo run --bin calc_distance_from_json_point "{\"lat\": 48.8534, \"lon\": 2.3488}" "{\"lat\": 48.8534, \"lon\": 2.3488}"
/// 0.0

// IMPORTS ===================================================================================================  IMPORTS
#[path = "gpx_utils.rs"]
mod gpx_utils;

use crate::gpx_utils::{Coord, calc_distance};
// END IMPORTS ==========================================================================================   END IMPORTS

// VARIABLES ================================================================================================ VARIABLE
// Type(s)

// Other(s)
// END VARIABLES ======================================================================================= END VARIABLES

// CODE ========================================================================================================= CODE
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        panic!("This binary takes two arguments: two jsons representing two Coord");
    }

    let coord_1: Coord = match serde_json::from_str(&args[1]) {
        Ok(coord) => coord,
        Err(_) => panic!("Could not parse the first argument as a Coord"),
    };

    let coord_2: Coord = match serde_json::from_str(&args[2]) {
        Ok(coord) => coord,
        Err(_) => panic!("Could not parse the second argument as a Coord"),
    };

    println!("{}", calc_distance(coord_1, coord_2, Some(true)));
}

// END CODE =======================================================================================  END COMPONENT

//
// * End of file /calc_distance_from_json_point.rs
//
