use haversine_redux::{Location, Unit};

fn main() {
    let start = Location::new(38.898556, -77.037852);
    let end = Location::new(38.897147, -77.043934);
    
    let km = start.distance_to(&end, Unit::Kilometer);
    let miles = start.distance_to(&end, Unit::Mile);

    println!("Distance: {} km", &km);
    println!("Distance: {} miles", &miles);
}