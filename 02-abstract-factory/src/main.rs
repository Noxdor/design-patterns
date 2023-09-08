mod furniture;
mod factories;
mod product_families;

use factories::*;

fn main() {
    let factories: Vec<Box<dyn FurnitureFactory>> = vec![
        Box::new(VictorianFurnitureFactory {}),
        Box::new(ArtDecoFurnitureFactory {}),
        Box::new(ModernFurnitureFactory {})
    ];

    for factory in factories {
        let chair = factory.create_chair();
        let sofa = factory.create_sofa();
        let coffee_table = factory.create_coffee_table();

        chair.sit_on();
        sofa.watch_tv();
        coffee_table.eat_dinner();
        println!("");
    }
}
