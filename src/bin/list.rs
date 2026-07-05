#[derive(Debug)]
struct Node<T>
{
  value:T,
  next:Option<Box<Node<T>>>
}
impl <T> Node<T> {
    fn new(value:T)->Self{
      Node{value,next:None} //默认为空
    }
}
struct LinkList<T>
{
  head:Option<Box<Node<T>>>,
  size:usize
}
impl <T> LinkList<T> {
  fn new()->Self {
    LinkList { head: None, size: 0 }
  }
    
}
//头插法

//尾插法 

fn main() {
    
}
