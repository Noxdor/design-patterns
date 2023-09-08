use crate::transport::{Truck, Ship, Transport};

pub trait Logistics {    
    fn plan_delivery(&self) {
        let t = self.create_transport();
        t.deliver();
    }
    fn calculate_costs(&self) {
        let t = self.create_transport();
        t.get_total();
    }
    fn create_transport(&self) -> Box<dyn Transport>;
}

pub struct RoadLogistics;

impl Logistics for RoadLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Truck { costs: 200 })
    }
}

pub struct SeaLogistics;

impl Logistics for SeaLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship {})
    }
}
