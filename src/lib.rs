use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

pub struct Heap<T> {
    n: i32,
    trees: i32,
    marks: i32,
    min: Option<Rc<Node<T>>>,
}

impl<T> Heap<T> {
    pub fn new() -> Self {
        Heap {
            n: 0,
            trees: 0,
            marks: 0,
            min: None,
        }
    }

    pub fn get_min(&self) -> Option<Rc<Node<T>>> {
        if let Some(min) = &self.min {
            Some(Rc::clone(min))
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: i32, value: T) {
        let node = Node::new(key, value);
        self.min = match &self.min {
            Some(min) => {
                min.concatenate(Rc::clone(&node));
                if node.get_key() < min.get_key() {
                    Some(node)
                } else {
                    Some(Rc::clone(min))
                }
            }
            None => Some(node),
        };
        self.n += 1;
    }

    pub fn minimum(&self) -> i32 {
        *self.get_min().unwrap().key.borrow()
    }

    pub fn union(&mut self, mut heap: Self) {
        if let Some(heap_min) = heap.get_min() {
            if let Some(self_min) = self.get_min() {
                if self_min.get_key() > heap_min.get_key() {
                    self.min = Some(Rc::clone(&heap_min));
                }
                self_min.concatenate(Rc::clone(&heap_min));
            } else {
                self.min = Some(heap_min);
            }
            heap.min = None;
        }
    }

    pub fn print(&self) {
        if let Some(min) = &self.min {
            min.print(self);
        } else {
            println!("none");
        }
        println!("");
    }
}

pub struct Node<T> {
    key: RefCell<i32>,
    value: T,
    // 循環参照を避けるために一方向はRcポインタ、もう一方はWeakポインタを使用
    parent: RefCell<Option<Weak<Node<T>>>>,
    child: RefCell<Option<Rc<Node<T>>>>,
    right: RefCell<Option<Rc<Node<T>>>>,
    left: RefCell<Option<Weak<Node<T>>>>,
    degree: RefCell<i32>,
    is_marked: RefCell<bool>,
}

impl<T> Node<T> {
    pub fn new(key: i32, value: T) -> Rc<Self> {
        let node = Rc::new(Node {
            key: RefCell::new(key),
            value: value,
            parent: RefCell::new(None),
            child: RefCell::new(None),
            right: RefCell::new(None),
            left: RefCell::new(None),
            degree: RefCell::new(0),
            is_marked: RefCell::new(false),
        });

        node.set_right(Rc::clone(&node));
        node.set_left(Rc::downgrade(&node));

        node
    }

    pub fn print(&self, Heap: &Heap<T>) {
        print!("{} ", *self.key.borrow(),);
        let right = self.get_right().unwrap();
        let start = if let Some(parent) = self.get_parent() {
            parent.get_child().unwrap()
        } else {
            Heap.get_min().unwrap()
        };
        if Rc::ptr_eq(&start, &right) == false {
            right.print(Heap);
        } else {
            print!("|");
        }
        if let Some(child) = self.get_child() {
            println!("");
            child.print(Heap);
        } else {
        }
    }

    pub fn get_key(&self) -> i32 {
        *self.key.borrow()
    }

    fn set_right(&self, node: Rc<Self>) {
        *self.right.borrow_mut() = Some(node);
    }

    fn get_right(&self) -> Option<Rc<Self>> {
        Node::from_ref_option_rc(self.right.borrow())
    }

    fn clear_right(&self) {
        *self.right.borrow_mut() = None;
    }

    fn set_left(&self, node: Weak<Self>) {
        *self.left.borrow_mut() = Some(node);
    }

    fn get_left(&self) -> Option<Rc<Self>> {
        Node::from_ref_option_weak(self.left.borrow())
    }

    fn clear_left(&self) {
        *self.left.borrow_mut() = None;
    }

    fn set_parent(&self, node: Weak<Self>) {
        *self.parent.borrow_mut() = Some(node);
    }

    fn get_parent(&self) -> Option<Rc<Self>> {
        Node::from_ref_option_weak(self.parent.borrow())
    }

    fn clear_parent(&self) {
        *self.parent.borrow_mut() = None;
    }

    fn set_child(&self, node: Rc<Self>) {
        *self.child.borrow_mut() = Some(node);
    }

    fn get_child(&self) -> Option<Rc<Self>> {
        Node::from_ref_option_rc(self.child.borrow())
    }

    fn clear_child(&self) {
        *self.child.borrow_mut() = None;
    }

    fn from_ref_option_rc(node: Ref<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        if let Some(ref n) = *node {
            Some(Rc::clone(n))
        } else {
            None
        }
    }

    fn from_borrowed_option_rc(node: &Option<Rc<Self>>) -> Option<Rc<Self>> {
        if let &Some(ref n) = node {
            Some(Rc::clone(n))
        } else {
            None
        }
    }

    fn from_ref_option_weak(node: Ref<Option<Weak<Self>>>) -> Option<Rc<Self>> {
        if let Some(ref n) = *node {
            Weak::upgrade(n)
        } else {
            None
        }
    }

    fn concatenate(&self, node: Rc<Self>) {
        let self_rc = self.get_left().unwrap().get_right().unwrap();
        let node_left = node.get_left().unwrap();
        let self_left = self.get_left().unwrap();

        node_left.set_right(Rc::clone(&self_rc));
        self.set_left(Rc::downgrade(&node_left));

        node.set_left(Rc::downgrade(&self_left));
        self_left.set_right(Rc::clone(&node));
    }
}

pub struct NodeIterator<T> {
    first: Rc<Node<T>>,
    current: Option<Rc<Node<T>>>,
    first_seen: bool,
}

impl<T> NodeIterator<T> {
    fn new(node: Rc<Node<T>>) -> Self {
        NodeIterator {
            first: Rc::clone(&node),
            current: Some(Rc::clone(&node)),
            first_seen: false,
        }
    }
}

impl<T> Iterator for NodeIterator<T> {
    type Item = Rc<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = Node::from_borrowed_option_rc(&self.current)?;

        if self.first_seen && Rc::ptr_eq(&current, &self.first) {
            return None;
        } else if Rc::ptr_eq(&current, &self.first) {
            self.first_seen = true;
        }

        self.current = current.get_right();

        Some(current)
    }
}
