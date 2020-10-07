use std::fmt;

pub struct Stack<T> {
    capacity: usize,
    size: usize,
    data: Vec<T>,
}

impl<T: fmt::Debug> Stack<T>{
    pub fn new(capacity: usize) -> Stack<T>{
        let data: Vec<T> = Vec::with_capacity(capacity);
        Stack{capacity, size:(0), data}
    }

    pub fn add(&mut self, data: T){
        if self.size == self.capacity{
            println!("Stack overflow! {:?} won't be added", data);
        }
        else{
            println!("Adding new element {:?}", data);
            &self.data.push(data);
            self.size += 1;
        }
    }

    pub fn pop(&mut self){
        println!("Deleting {:?}", &self.data[self.size-1]);
        &self.data.pop();
        self.size -= 1;
    }

    pub fn print(&self){
        println!("Printing from first element");
        for elem in &self.data{
            print!("{:?} ", elem);
        }
        println!("");
    }
}
