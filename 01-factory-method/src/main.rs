use logistics::Logistics;

mod transport;
mod logistics;

fn main() {

    let logist: Box<dyn Logistics> = match rand::random::<bool>() {
        true => Box::new(logistics::RoadLogistics{}),
        false => Box::new(logistics::SeaLogistics{})
    };

    logist.plan_delivery();
    logist.calculate_costs();
}
