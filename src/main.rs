extern crate fibonacci_heap;

fn main() {
    let mut heap = fibonacci_heap::Heap::new();
    heap.insert(8);
    heap.insert(10);
    heap.insert(3);
    heap.insert(5);
    heap.print();

    let mut heap2 = fibonacci_heap::Heap::new();
    heap2.insert(4);
    heap2.insert(6);
    heap2.insert(30);
    heap.union(heap2);
    heap.print();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
