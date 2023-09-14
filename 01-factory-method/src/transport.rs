
pub trait Transport {
    fn deliver(&self);
    fn get_total(&self);
}

pub struct Truck {
    costs: u32
}

impl Truck {
    pub fn new() -> Self {
        Self {
            costs: 220
        }
    }
}

impl Transport for Truck {
    fn deliver(&self) {
        println!("Brum brum, delivering by truck.");
    }
    fn get_total(&self) {
        println!("Variable truck costs are: {}k", self.costs);
    }
}

pub struct Ship;

impl Ship {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Transport for Ship {
    fn deliver(&self) {
        println!("Iceberg ahead! Delivering by ship.");
    }
    fn get_total(&self) {
        println!("Fixed ship costs are: 500k");
    }
}
