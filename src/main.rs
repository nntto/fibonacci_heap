extern crate fibonacci_heap;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

fn main() {
    let mut heap = fibonacci_heap::Heap::new();

    heap.insert(23, 1);
    heap.insert(7, 1);
    heap.insert(21, 1);

    let node3 = fibonacci_heap::Node::new(3, 0);
    let node18 = fibonacci_heap::Node::new(18, 0);
    node18.add_child(fibonacci_heap::Node::new(39, 1));
    node3.add_child(node18);
    node3.add_child(fibonacci_heap::Node::new(52, 1));
    node3.add_child(fibonacci_heap::Node::new(28, 1).add_child(fibonacci_heap::Node::new(41, 1)));
    heap.insert_node(node3);

    heap.insert_node(fibonacci_heap::Node::new(17, 1).add_child(fibonacci_heap::Node::new(30, 1)));
    let node24 = fibonacci_heap::Node::new(3, 0);
    node24.add_child(fibonacci_heap::Node::new(46, 1));
    node24.add_child(fibonacci_heap::Node::new(26, 1).add_child(fibonacci_heap::Node::new(35, 1)));
    heap.insert_node(node24);

    println!("before extract_min");
    heap.print();

    println!("extract_min min={}", heap.extract_min().unwrap().get_key());
    heap.print();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
