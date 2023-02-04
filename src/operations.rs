pub trait Operation {
    fn calculate(&self, a: i32, b: i32) -> i32;
}

pub struct Addition;
pub struct Subtraction;


impl Operation for Addition {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

impl Operation for Subtraction {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}