use std::vec;

#[derive(Debug)]
pub struct stack<T> {
    data: vec<T>,
}
impl<T> stack<T> {
    pub fn new() -> Self {
        stack { data: Vec::new() }
    }
}
fn main() {}
