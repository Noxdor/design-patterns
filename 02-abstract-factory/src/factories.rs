use crate::furniture::*;
use crate::product_families::victorian::*;
use crate::product_families::art_deco::*;
use crate::product_families::modern::*;

pub trait FurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair>;
    fn create_sofa(&self) -> Box<dyn Sofa>;
    fn create_coffee_table(&self) -> Box<dyn CoffeeTable>;
}

pub struct VictorianFurnitureFactory;

impl FurnitureFactory for VictorianFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(VictorianChair {}) 
    }
    fn create_sofa(&self) -> Box<dyn Sofa> {
        Box::new(VictorianSofa {}) 
    }
    fn create_coffee_table(&self) -> Box<dyn CoffeeTable> {
        Box::new(VictorianCoffeeTable {})
    }
}

pub struct ArtDecoFurnitureFactory;

impl FurnitureFactory for ArtDecoFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(ArtDecoChair {}) 
    }
    fn create_sofa(&self) -> Box<dyn Sofa> {
        Box::new(ArtDecoSofa {}) 
    }
    fn create_coffee_table(&self) -> Box<dyn CoffeeTable> {
        Box::new(ArtDecoCoffeeTable {})
    }
}

pub struct ModernFurnitureFactory;

impl FurnitureFactory for ModernFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(ModernChair {}) 
    }
    fn create_sofa(&self) -> Box<dyn Sofa> {
        Box::new(ModernSofa {}) 
    }
    fn create_coffee_table(&self) -> Box<dyn CoffeeTable> {
        Box::new(ModernCoffeeTable {})
    }
}
