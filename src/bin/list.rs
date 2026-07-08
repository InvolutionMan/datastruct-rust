use std::{hash::RandomState, ops::Index, sync::mpsc::TrySendError};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None } //默认为空
    }
}
struct LinkList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}
impl<T> LinkList<T> {
    fn new() -> Self {
        LinkList {
            head: None,
            size: 0,
        }
    }
    //头插法
    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            next: self.head.take(),
            value,
        });
        self.head = Some(new_node);
        self.size += 1;
    }
    //尾插法
    fn push_tail(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });
        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => {
                while let Some(ref mut next) = current.next {
                    //ref  借可变引用
                    current = next;
                }
                current.next = Some(new_node);
            }
        }
        self.size += 1;
    }
    // 打印链表
    fn print_list(&self)
    where
        T: std::fmt::Display,
    {
        let mut temp = &self.head;
        while let Some(node) = temp {
            print!("{}->", node.value);
            temp = &node.next;
        }
        println!("None");
        println!("链表的长度为{}", &self.size);
    }
    // 反转链表
    fn reverselist (&mut self) where T:std::fmt::Display
    {
        let mut prev=None;
        let mut current=self.head.take();
        while let Some(mut node)=current
        {
          let next=node.next.take();
          node.next=prev;
          prev=Some(node);
          current=next;
        }
        self.head=prev;
        
    }
    // 按值查找
    fn find_by_value(&self,value: T) ->bool
    where T:PartialEq,
    {
      let mut current=&self.head;
      while let Some(mut node) =current.as_ref() 
      {
          if node.value==value
          {
            return true;
          }
          current=&node.next;
      }
      return false;
    }
    // 按索引查找
    fn find_by_index(&self,index:usize) ->bool
    {
      let mut current=&self.head;
      let mut i=0;
      while let Some(node) =current   
      {
          if i==index
          {
            return true;
          }
          current=&node.next;
          i+=1;
      }
      return false;
    }
    //通过值删除节点
    fn delete_node_by_value(&mut self,value: &T) ->bool where T:PartialEq
    {  
      let mut current=&mut self.head;
      if let Some(node)=self.head.as_ref() //as_ref 不可变引用
      {
        if node.value==*value
        {
          self.head = self.head.take().unwrap().next;
          self.size-=1;
          return true;
        }
      }
      while let Some(node)=current.as_mut() // as_mut可变引用
      {
        if let Some(next_node)=node.next.as_ref()
        {
          if next_node.value==*value
          {
            let mut remove=node.next.take().unwrap();
            node.next=remove.next;
            self.size-=1;
            return true;
          }
        }
        current=&mut node.next;
      }
      return false;
    }
    //通过索引删除节点
}

fn main() {
  let mut list=LinkList::new();
  list.push_front(1);
  list.push_tail(2);
  list.print_list();
  println!("是否找到节点，{}",list.find_by_value(1));
  println!("是否找到节点，{}",list.find_by_index(1));
  
}
