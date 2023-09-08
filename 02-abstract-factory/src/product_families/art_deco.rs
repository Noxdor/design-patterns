use crate::furniture::*;

pub struct ArtDecoChair;

impl Chair for ArtDecoChair {
    fn sit_on(&self) {
        println!("Sitting on art deco chair."); 
    }
    fn has_legs(&self) -> u8 {
        1
    }
}

pub struct ArtDecoSofa;

impl Sofa for ArtDecoSofa {
    fn sit_on(&self) {
       println!("Sitting on art deco couch.") ;
    }
    fn lay_on(&self) {
        println!("Laying on art deco couch.");
    }
    fn watch_tv(&self) {
        println!("Watching artsy tv.");
    }
}

pub struct ArtDecoCoffeeTable;

impl CoffeeTable for ArtDecoCoffeeTable {
    fn drink_coffee(&self) {
        println!("Drinking coffee art deco style.");
    }
    fn eat_dinner(&self) {
        println!("Eating artsy dinner.");
    }
}
