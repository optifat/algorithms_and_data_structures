use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<T>>>;

struct Node<T> {
    data: T,
    next: Link<Node<T>>,
    prev: Link<Node<T>>
}

pub struct List<T>{
    head: Link<Node<T>>,
    tail: Link<Node<T>>,
    size: usize
}

impl<T: fmt::Debug> Node<T>{
    fn new(data: T, next: Link<Node<T>>, prev: Link<Node<T>>) -> Node<T>{
        Node{data, next, prev}
    }
}

impl<T: fmt::Debug> List<T>{
    pub fn new() -> List<T>{
        List{head: None, tail: None, size: 0}
    }

    pub fn push_back(&mut self, data: T){
        println!("Adding new element {:?}", data);
        if self.size == 0{
            let new_node = Rc::new(RefCell::new(Node::new(data, None, None)));
            self.head = Some(new_node);
            self.tail = Some(Rc::clone(self.head.as_ref().unwrap()));
            self.size += 1;
        }
        else{
            let new_node = Rc::new(RefCell::new(Node::new(data, None, self.tail.clone())));
            {
                let mut ref_tail = self.tail.as_ref().unwrap().borrow_mut();
                ref_tail.next.replace(new_node);

            }
            let tmp = {
                Some(Rc::clone(self.tail.as_ref().unwrap().borrow().next.as_ref().unwrap()))
            };
            self.tail = tmp.clone();
            self.size += 1;
        }
    }

    pub fn print(&self){
        println!("Printing from first element");
        let mut current_node = self.head.clone();
        for _ in 0..self.size{
            current_node = {
                let current_node_borrowed = current_node.as_ref().unwrap().borrow();
                print!("{:?} ", current_node_borrowed.data);
                current_node_borrowed.next.clone()
            };
        }
        println!("");
    }
}
