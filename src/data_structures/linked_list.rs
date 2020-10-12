use std::fmt;

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
        println!("Adding new element {:?}", data);
        let new_node = Box::new(Node::new(data, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn print(&self){
        //let mut printing_stack = stack::Stack::<T>::new(self.size);
        println!("Printing list in reverse order");
        let mut current_node = self.head.as_ref();
        for _ in 0..self.size{
            print!("{:?} ", current_node.unwrap().data);
            current_node = current_node.unwrap().next.as_ref();
        }
        println!("");
    }
}
