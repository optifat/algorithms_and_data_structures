mod data_structures;

use crate::data_structures::{
    stack,
    queue
};

fn main() {

    let mut my_stack = stack::Stack::<i32>::new(4 as usize);
    for i in 1..6{
        my_stack.add(i)
    }
    my_stack.print();
    my_stack.pop();
    my_stack.print();
    my_stack.add(5);
    my_stack.print();

    println!("");

    let mut my_queue = queue::Queue::<i32>::new(4 as usize);
    for i in 1..6{
        my_queue.add(i)
    }
    my_queue.print();
    my_queue.pop();
    my_queue.print();
    my_queue.add(5);
    my_queue.print();
    my_queue.pop();
    my_queue.print();
}
