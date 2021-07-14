extern crate fibonacci_heap;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

fn main() {
    let mut heap = fibonacci_heap::Heap::new();

    heap.insert(23, 1, false);
    heap.insert(7, 1, false);
    heap.insert(21, 1, false);

    let node3 = fibonacci_heap::Node::new(3, 0, false);
    let node18 = fibonacci_heap::Node::new(18, 0, true);
    node18.add_child(fibonacci_heap::Node::new(39, 1, true));
    node3.add_child(Rc::clone(&node18));
    node3.add_child(fibonacci_heap::Node::new(52, 1, false));
    node3.add_child(
        fibonacci_heap::Node::new(28, 1, false).add_child(fibonacci_heap::Node::new(41, 1, false)),
    );
    heap.insert_node(node3);

    heap.insert_node(
        fibonacci_heap::Node::new(17, 1, false).add_child(fibonacci_heap::Node::new(30, 1, false)),
    );
    let node24 = fibonacci_heap::Node::new(24, 0, false);
    let node46 = fibonacci_heap::Node::new(46, 1, false);
    let node35 = fibonacci_heap::Node::new(35, 1, false);
    node24.add_child(Rc::clone(&node46));
    node24.add_child(fibonacci_heap::Node::new(26, 1, true).add_child(Rc::clone(&node35)));
    heap.insert_node(node24);
    heap.set_n(15);

    println!("initial state");
    heap.print();

    println!("extract_min min={}", heap.extract_min().unwrap().get_key());
    heap.print();

    println!("decrease_key 46 to 15");
    heap.decrease_key(Rc::clone(&node46), 15);
    heap.print();

    println!("decrease_key 35 to 5");
    heap.decrease_key(Rc::clone(&node35), 5);
    heap.print();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
