///
/// # GPX_utils.rs
/// This file contains functions for GPX manipulation.
///
/// ## Author
/// * Tom Planche - <github.com/tomPlanche>

// IMPORTS ===================================================================================================  IMPORTS
use std::fmt::Display;
use std::hash::Hash;

use serde::{Deserialize, Serialize};
// END IMPORTS ==========================================================================================   END IMPORTS

// VARIABLES ================================================================================================ VARIABLE
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ lat: {}, lon: {} }}", self.lat, self.lon)
    }
}

// Hash in order to use as a key in a HashMap
impl Hash for Coord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lat.to_bits().hash(state);
        self.lon.to_bits().hash(state);
    }
}

// PartialEq for the '==' operation.
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.lon == other.lon
    }
}

// PartialEq being implemented, Eq can be derived.
impl Eq for Coord {}

// END VARIABLES =======================================================================================  END VARIABLES

// FUNCTIONS ================================================================================================ FUNCTIONS
///
/// # calc_distance
/// Calculate the distance between two coordinates
/// using the Haversine formula.
///
/// ## Arguments
/// * `coord1(Coord)` - The first coordinate in decimal degrees.
/// * `coord2(Coord)` - The second coordinate in decimal degrees.
///
/// ## Returns
/// * `f64` - The distance between the two coordinates (in meters).
pub fn calc_distance(coord1: Coord, coord2: Coord, in_meter: Option<bool>) -> f64 {
    let Coord { lat: lat_1, lon: lon_1 } = coord1;
    let Coord { lat: lat_2, lon: lon_2 } = coord2;

    const EARTH_RADIUS: f64 = 6371.0; // km

    let delta_lat: f64 = (lat_2 - lat_1).to_radians();
    let delta_lon: f64 = (lon_2 - lon_1).to_radians();

    let lat_1: f64 = lat_1.to_radians();
    let lat_2: f64 = lat_2.to_radians();

    let a: f64 =
        (delta_lat / 2.0).sin()
        * (delta_lat / 2.0).sin()
        + (delta_lon / 2.0).sin()
        * (delta_lon / 2.0).sin()
        * lat_1.cos()
        * lat_2.cos();

    let c: f64 =
        2.0
        * a.sqrt()
        .atan2((1.0 - a).sqrt());

    EARTH_RADIUS * c * if in_meter.unwrap_or(false) { 1000.0 } else { 1.0 }
}

#[test]
fn test_calc_distance() {
    let coord1 = Coord {
        lat: 48.8534,
        lon: 2.3488,
    };

    let coord2 = Coord {
        lat: 48.8534,
        lon: 1.0,
    };

    assert_eq!(calc_distance(coord1, coord2, None), 98.6835497563641);
    assert_eq!(calc_distance(coord1, coord1, None), 0.0);
}

#[test]
fn test_coord_serialize() {
    let coord1 = Coord {
        lat: 48.8534,
        lon: 2.3488,
    };

    let coord1_str = serde_json::to_string(&coord1).unwrap();

    assert_eq!(coord1_str, "{\"lat\":48.8534,\"lon\":2.3488}");
}

#[test]
fn test_coord_deserialize() {
    let coord1 = Coord {
        lat: 48.8534,
        lon: 2.3488,
    };

    let coord1_str = serde_json::to_string(&coord1).unwrap();

    let coord1_deserialized: Coord = serde_json::from_str(&coord1_str).unwrap();

    assert_eq!(coord1_deserialized, coord1);
}

// END FUNCTIONS =======================================================================================  END FUNCTIONS

//
// * End of file file_utils.rs
//
