use std::fmt;

pub struct Queue<T> {
    capacity: usize,
    size: usize,
    start: usize,
    data: Vec<T>,
}

impl <T: fmt::Debug> Queue<T>{
    pub fn new(capacity: usize) -> Queue<T>{
        let data: Vec<T> = Vec::with_capacity(capacity);
        Queue{capacity, size: 0, start: 0, data}
    }

    pub fn add(&mut self, data: T){
        if self.size == self.capacity{
            println!("Queue is full! {:?} won't be added", data);
        }
        else if self.data.len() < self.capacity{
            println!("Adding new element {:?} was added", data);
            self.data.push(data);
            self.size += 1;
        }
        else{
            println!("Adding new element {:?} was added", data);
            let index: usize = (self.start+self.size)%self.capacity;
            self.data[index] = data;
            self.size += 1;
        }
    }

    pub fn pop(&mut self){
        println!("Deleting {:?}", &self.data[self.start]);
        self.start = (self.start+1)%self.capacity;
        self.size -= 1;
    }

    pub fn print(&self){
        println!("Printing from first element");
        for i in 0..self.size{
            print!("{:?} ", &self.data[(self.start+i)%self.capacity]);
        }
        println!("");
    }
}
