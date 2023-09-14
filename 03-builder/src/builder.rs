use crate::vehicle::{Engine, Car, Motorbike};

pub trait Builder {
    type Product;
    fn reset(&mut self);
    fn set_seats(self, count: u8) -> Self;
    fn set_engine(self, engine: Engine) -> Self;
    fn set_kilometer_counter(self, km: Option<u32>) -> Self;
    fn build(self) -> Self::Product;
}

#[derive(Default, Debug)]
pub struct CarBuilder {
    car: Car,
}

impl Builder for CarBuilder {
    type Product = Car;
    fn reset(&mut self) {
        self.car = Car::default();
    }
    fn set_seats(mut self, count: u8) -> Self {
        self.car.seats = count;
        self
    }
    fn set_engine(mut self, engine: Engine) -> Self {
        self.car.engine = engine;
        self
    }

    fn set_kilometer_counter(mut self, km: Option<u32>) -> Self {
        self.car.kilometers = km.unwrap_or(0);
        self
    }

    fn build(self) -> Self::Product {
        self.car
    }
}

#[derive(Default)]
pub struct MotorbikeBuilder {
    motorbike: Motorbike,
}

impl Builder for MotorbikeBuilder {
    type Product = Motorbike;
    fn reset(&mut self) {
        self.motorbike = Motorbike::default();
    }
    fn set_seats(mut self, count: u8) -> Self {
        self.motorbike.seats = count;
        self
    }
    fn set_engine(mut self, engine: Engine) -> Self {
        self.motorbike.engine = engine;
        self
    }

    fn set_kilometer_counter(mut self, km: Option<u32>) -> Self {
        self.motorbike.kilometers = km.unwrap_or(0);
        self
    }

    fn build(self) -> Self::Product {
        self.motorbike
    }
}
