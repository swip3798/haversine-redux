#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(not(feature = "no_std"))]
mod math;
#[cfg(not(feature = "no_std"))]
use math::{atan2, cos, sin, sqrt};

#[cfg(feature = "no_std")]
use libm::{atan2, cos, sin, sqrt};

fn squared(x: f64) -> f64 {
    x * x
}


const KILOMETERS: f64 = 6371.0;
const MILES: f64 = 3960.0;
// Definition of PI from the std library
const PI: f64 = 3.14159265358979323846264338327950288f64;

/// All supported units of distance
#[derive(Debug, Clone)]
pub enum Unit {
    Kilometer,
    Mile,
    CustomSphere(f64),
}

impl Unit {
    fn radius(&self) -> f64 {
        match self {
            Unit::Kilometer => KILOMETERS,
            Unit::Mile => MILES,
            Unit::CustomSphere(r) => *r,
        }
    }
}

/// A location represented with a latitude and longitude
#[derive(Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    /// Creates a new location
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /**
        Calculates the distance between two locations and returns the distance in the assigned unit

        ```rust,no_run
        use haversine_redux::{Location, Unit};

        fn main() {
            let start = Location::new(38.898556, -77.037852);
            let end = Location::new(38.897147, -77.043934);
            
            let km = start.distance_to(&end, Unit::Kilometer);
            let miles = start.distance_to(&end, Unit::Mile);
        }
    */
    pub fn distance_to(&self, other: &Location, unit: Unit) -> f64 {
        let r = unit.radius();

        let d_lat: f64 = (other.latitude - self.latitude) * PI / 180.0;
        let d_lon: f64 = (other.longitude - self.longitude) * PI / 180.0;

        let lat1: f64 = self.latitude * PI / 180.0;
        let lat2: f64 = other.latitude * PI / 180.0;

        let a: f64 =
            squared(sin(d_lat / 2.0)) + squared(sin(d_lon / 2.0)) * (cos(lat1)) * (cos(lat2));
        let c: f64 = 2.0 * atan2(sqrt(a), sqrt(1.0 - a));

        r * c
    }

    /**
        Helper function, equivalent of
        ```compile_fail
        start.distance_to(&end, Unit::Mile);
        ```
     */
    pub fn miles_to(&self, other: &Location) -> f64 {
        self.distance_to(other, Unit::Mile)
    }

    /**
        Helper function, equivalent of
        ```compile_fail
        start.distance_to(&end, Unit::Kilometer);
        ```
     */
    pub fn kilometers_to(&self, other: &Location) -> f64 {
        self.distance_to(other, Unit::Kilometer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haversine_distance_in_miles() {
        let start = Location::new(38.898556, -77.037852);
        let end = Location::new(38.897147, -77.043934);
        assert_eq!(0.341336828310639, start.miles_to(&end))
    }

    #[test]
    fn haversine_distance_in_kilometers() {
        let start = Location::new(38.898556, -77.037852);
        let end = Location::new(38.897147, -77.043934);
        assert_eq!(0.5491557912038084, start.kilometers_to(&end))
    }

    #[test]
    fn helper_function_correctness() {
        let start = Location::new(38.898556, -77.037852);
        let end = Location::new(38.897147, -77.043934);
        assert_eq!(start.distance_to(&end, Unit::Mile), start.miles_to(&end));
        assert_eq!(start.distance_to(&end, Unit::Kilometer), start.kilometers_to(&end));
    }
}
