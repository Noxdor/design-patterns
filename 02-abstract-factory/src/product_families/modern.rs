use crate::furniture::*;

pub struct ModernChair;

impl Chair for ModernChair {
    fn sit_on(&self) {
        println!("Sitting on modern chair."); 
    }
    fn has_legs(&self) -> u8 {
        4 
    }
}

pub struct ModernSofa;

impl Sofa for ModernSofa {
    fn sit_on(&self) {
       println!("Sitting on modern couch.") ;
    }
    fn lay_on(&self) {
        println!("Laying on modern couch.");
    }
    fn watch_tv(&self) {
        println!("Watching hip tv.");
    }
}

pub struct ModernCoffeeTable;

impl CoffeeTable for ModernCoffeeTable {
    fn drink_coffee(&self) {
        println!("Drinking caffee latte.");
    }
    fn eat_dinner(&self) {
        println!("Eating something vegan for dinner, probably.");
    }
}
