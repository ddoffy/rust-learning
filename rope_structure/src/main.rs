use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    weight: usize,
    left: Option<Box<Rope>>,
    right: Option<Box<Rope>>,
}

fn main() {
    println!("Hello, world!");
}
