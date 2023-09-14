#[derive(Debug)]
pub enum Engine {
    V4(u32),
    V8(u32),
    V16(u32),
    Electric(u32)
}

impl Default for Engine {
    fn default() -> Self {
        Engine::V4(80)
    }
}

#[derive(Default, Debug)]
pub struct Car {
    pub engine: Engine,
    pub kilometers: u32,
    pub seats: u8
}

#[derive(Default, Debug)]
pub struct Motorbike {
    pub engine: Engine,
    pub kilometers: u32,
    pub seats: u8
}
