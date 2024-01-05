use std::fmt::Debug;
#[derive(Debug)]
pub struct Stack<T> {
   pub data: Vec<T>,
}
impl<T> Stack<T> 
where 
    T:Debug,
{
    pub fn new(max_size: usize) -> Self {
        Stack {
            data: Vec::with_capacity(max_size),
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn push(&mut self, item: T, max_size: usize) {
        if self.data.len() == max_size {
            println!("Can't push more than {}", max_size);
        } else {
            self.data.push(item);
            println!("Stack: {:?}", self.data);
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
