use std::fmt;
use std::ptr;

use crate::data_structures::stack;

type Link<T> = Option<Box<T>>;

struct Node<T> {
    data: T,
    next: Link<Node<T>>,
}

pub struct List<T>{
    head: Link<Node<T>>,
    size: usize
}

impl<T: fmt::Debug> Node<T>{
    fn new(data: T, next: Link<Node<T>>) -> Node<T>{
        Node{data, next}
    }
}

impl<T: fmt::Debug> List<T>{
    pub fn new() -> List<T>{
        List{head: None, size: 0}
    }

    pub fn push_back(&mut self, data: T){
        let new_node = Box::new(Node::new(data, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    pub fn print(&self){
        //let mut printing_stack = stack::Stack::<T>::new(self.size);
        let mut current_node = self.head.as_ref().unwrap();
        for _ in 0..self.size{
            print!("{:?} ", current_node.data);
            //current_node = current_node.next.as_ref().unwrap();
        }
        print!("{:?}", self.head.as_ref().unwrap().next.as_ref().unwrap().data);
        println!("");
    }
    /*
    pub fn add(&mut self, data: T){
        if self.size == 0{
            self.head = &mut Node::new(data, ptr::null_mut());
            self.tail = self.head;
            self.size += 1;
        }
        else{
            unsafe{
                (*self.tail).next =  &mut Node::new(data, self.tail);
                self.tail = (*self.tail).next;
                //println!("{:?}", (*self.tail).data);
            }
            self.size += 1
        }
    }

    pub fn pop(&mut self){
        unsafe{
            self.tail = (*self.tail).prev;
        }
        self.size -= 1;
    }

    pub fn print(&self){
        let mut current_node = self.head;
        unsafe{
            for _ in 0..self.size{
                print!("{:?} ", (*current_node).data);
                current_node = (*current_node).next;
            }
            println!("");
        }
    }
    */
}
