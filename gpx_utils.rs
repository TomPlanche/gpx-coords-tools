///
/// # GPX_utils.rs
/// This file contains functions for GPX manipulation.
///
/// ## Author
/// * Tom Planche - <github.com/tomPlanche>

use std::fmt::Display;
use std::hash::Hash;
use serde::{Serialize};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Coord {
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

impl Serialize for Coord {
    ///
    /// # serialize
    /// This function will serialize a Coord struct.
    ///
    /// ## Arguments
    /// * `serializer(S)` - The serializer
    ///
    /// ## Returns
    /// * `Result<S::Ok, S::Error>` - The result of the serialization
    ///
    /// ## Example
    /// ```
    /// use gpx_utils::Coord;
    /// use serde_json::json;
    ///
    /// let coord: Coord = Coord {
    ///    lat: 48.8534,
    ///   lon: 2.3488,
    /// };
    ///
    /// let serialized_coord = serde_json::to_string(&coord).unwrap();
    ///
    /// assert_eq!(serialized_coord, json!({
    ///    "lat": 48.8534,
    ///   "lon": 2.3488,
    /// }).to_string());
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut state = serializer.serialize_struct("Coord", 2)?;

        state.serialize_field("lat", &self.lat)?;
        state.serialize_field("lon", &self.lon)?;
        state.end()
    }
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
        return self.lat == other.lat && self.lon == other.lon;
    }
}

// PartialEq being implemented, Eq can be derived.
impl Eq for Coord {}

///
/// # calc_distance
/// This function will calculate the distance between two coordinates
/// using the Haversine formula.
///
/// ## Arguments
/// * `coord1(Coord)` - The first coordinate in decimal degrees.
/// * `coord2(Coord)` - The second coordinate in decimal degrees.
///
/// ## Returns
/// * `f64` - The distance between the two coordinates (in meters).
pub(crate) fn calc_distance(coord1: Coord, coord2: Coord, in_meter: Option<bool>) -> f64 {
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

    return EARTH_RADIUS * c * if in_meter.unwrap_or(true) { 1000.0 } else { 1.0 };
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

    assert_eq!(calc_distance(coord1, coord2, Some(true)), 98.6835497563641);
    assert_eq!(calc_distance(coord1, coord1, None), 0.0);
}
