use std::{collections::hash_map::Values, vec};

#[derive(Debug)]
struct stack<T> 
{
    data: Vec<T>,
}
impl<T> stack<T> 
{
    fn new() -> Self 
    {
        stack { data: Vec::new() }
    }
    fn push(&mut self,value:T) 
    {
      self.data.push(value);
    }
    fn pop(&mut self) 
    {
      self.data.pop();
    }
    
}


fn main() {}
