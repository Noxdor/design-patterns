
pub trait Chair {
    fn sit_on(&self);
    fn has_legs(&self) -> u8;
}

pub trait Sofa {
    fn sit_on(&self);
    fn lay_on(&self);
    fn watch_tv(&self);
}

pub trait CoffeeTable {
    fn drink_coffee(&self);
    fn eat_dinner(&self);
}
