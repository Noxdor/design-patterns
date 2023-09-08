
pub trait Transport {
    fn deliver(&self);
    fn get_total(&self);
}

pub struct Truck {
    pub costs: u32
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

impl Transport for Ship {
    fn deliver(&self) {
        println!("Iceberg ahead! Delivering by ship.");
    }
    fn get_total(&self) {
        println!("Fixed ship costs are: 500k");
    }
}
