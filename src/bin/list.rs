use std::thread::current;

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
            }
        }
        self.size += 1;
    }
    // 打印链表
    // 反转链表
    // 按值查找
    // 按索引查找
    // 插入节点
}

fn main() {}
