use crate::furniture::*;

pub struct VictorianChair;

impl Chair for VictorianChair {
    fn sit_on(&self) {
        println!("Sitting on victorian chair."); 
    }
    fn has_legs(&self) -> u8 {
        4 
    }
}

pub struct VictorianSofa;

impl Sofa for VictorianSofa {
    fn sit_on(&self) {
       println!("Sitting on victorian couch.") ;
    }
    fn lay_on(&self) {
        println!("Laying on victorian couch.");
    }
    fn watch_tv(&self) {
        println!("Watching bridgeton.");
    }
}

pub struct VictorianCoffeeTable;

impl CoffeeTable for VictorianCoffeeTable {
    fn drink_coffee(&self) {
        println!("Drinking tea, actually. Earl Gray, precisely");
    }
    fn eat_dinner(&self) {
        println!("Eating supper.");
    }
}
