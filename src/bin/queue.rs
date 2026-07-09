use std::collections::VecDeque;
//双端队列
struct queue<T> {
    data: VecDeque<T>,
}
impl<T> queue<T> {
    fn new() -> Self {
        queue {
            data: VecDeque::new(),
        }
    }
    fn push(&mut self, value: T) {
        self.data.push_back(value);
    }
    fn pop(&mut self) {
        self.data.pop_front();
    }
}
struct CircularQueue<T> {
    data: Vec<Option<T>>,
    front: usize,
    rear: usize,
    capacity: usize, //队列容量
}
impl<T: Clone> CircularQueue<T> {
    // 创建指定容量的循环队列
    fn new(capacity: usize) -> Self {
        CircularQueue {
            data: vec![None; capacity + 1], // 多分配一个单元
            front: 0,
            rear: 0,
            capacity: capacity + 1,
        }
    }
    fn is_empty(&mut self) -> bool {
        self.front == self.rear
    }
    fn is_full(&mut self) -> bool {
        (self.rear + 1) % self.capacity == self.front
    }
    fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.is_full() {
            println!("队列满了");
        }
        self.data[self.rear] = Some(value);
        self.rear = (self.rear + 1) % self.capacity;
        println!("入队成功");
        Ok(())
    }
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            println!("队列为空");
            return None;
        }
        let mut x = self.data[self.front].take();
        self.front = (self.front + 1) % self.capacity;
        return x;
    }
}

fn main() {}
