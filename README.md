# Haversine Formular in Rust

[![Documentation](https://docs.rs/haversine-redux/badge.svg)](https://docs.rs/haversine-redux)
[![Crates.io](https://img.shields.io/crates/v/haversine-redux.svg)](https://crates.io/crates/haversine-redux)
[![License](https://img.shields.io/crates/l/haversine-redux.svg)](https://github.com/fltk-rs/fltk-rs/blob/master/LICENSE)
[![CI](https://github.com/swip3798/haversine-redux/actions/workflows/rust.yml/badge.svg)](https://github.com/swip3798/haversine-redux/actions/workflows/rust.yml)

This is a small library to implement the haversine formular in rust. There is already an haversine crate out there (you can find it [here](https://crates.io/crates/haversine)), but that library hasn't been updated since 2015 and because of that, it has some issues, especially the missing documentation and a little bit tedious and inefficient API. 

Because of that, here's a rewrite, with a more intuitive interface and optional support for `#![no_std]`.

## Usage

Add haversine-redux as a dependency to your project Cargo.toml file:

```toml
[dependencies]
haversine-redux = "0.2"
```

Example usage:
```rust
use haversine_redux::{Location, Unit};

fn main() {
    let start = Location::new(38.898556, -77.037852);
    let end = Location::new(38.897147, -77.043934);
    
    // Kilometers
    let km = start.distance_to(&end, Unit::Kilometer);
    // OR
    let km = start.kilometers_to(&end);

    // Miles
    let miles = start.distance_to(&end, Unit::Mile);
    // OR
    let miles = start.miles_to(&end);

    // To use the haversine formular with a custom sphere which is not the earth use the CustomSphere unit and add the radius
    let custom = start.distance_to(&end, Unit::CustomSphere(50.0));
}
```

## Optional support for `#![no_std]`
To enable no_std support, just add the `no_std` feature to the haversine-redux dependency:
```toml
[dependencies]
haversine-redux = {version = "0.2", features = ["no_std"]}
```
This will load the dependency [libm](https://crates.io/crates/libm) for the implementations of mathematical functions like sin, cos or atan2.

## Licensing
This library is dual-licensed under the MIT and the Apache 2.0 License.
