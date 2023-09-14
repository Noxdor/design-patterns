mod builder;
mod vehicle;

use builder::Builder;

use crate::builder::{MotorbikeBuilder, CarBuilder};

fn main() {
    let builder = MotorbikeBuilder::default();

    let vehicle = builder
        .set_seats(4)
        .set_engine(vehicle::Engine::Electric(200))
        .set_kilometer_counter(Some(20))
        .build();

    println!("{vehicle:?}");
}
